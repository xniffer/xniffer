extern crate comfy_table;
extern crate rayon;
extern crate clap;

use clap::{Arg, Command};
use comfy_table::Table;
use rayon::prelude::*;
use std::char::decode_utf16;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> Result<(), Box<dyn Error>> {
	println!("⎯⎯⎯ xniffer v{} ⎯⎯⎯", std::env!("CARGO_PKG_VERSION"));

	let matches = Command::new("xniffer")
		.version(CARGO_PKG_VERSION)
		.author(CARGO_PKG_AUTHORS)
		.about("A simple exif sniffer written in Rust")
		.arg(Arg::new("PATHS")
			.required(true)
			.help("Specify paths")
			.takes_value(true)
			.multiple_values(true)
		)
		.arg(Arg::new("RAW")
			.help("show raw data")
			.short('r')
			.long("raw")
			.takes_value(false)
		)
		.after_help("https://github.com/3top1a/xniffer")
		.get_matches();

	let files: Vec<String> = convert_folder_input_into_files_within(
		matches.values_of_t("PATHS").unwrap_or_else(|e| e.exit())
	);

	// Logic
	files.par_iter().for_each(|x| parse(x, !matches.is_present("RAW")));

	Ok(())
}

fn parse(name: &String, show_raw: bool) {
	// Try
	let metadata = rexiv2::Metadata::new_from_path(name);
	if metadata.is_err() {
		println!("{} Could not be parsed.", name);
		return;
	}

	let meta = metadata.unwrap();
	let mut data: Vec<Data> = Vec::new();

	// Exif tags
	if meta.has_exif() {
		let exifs = meta.get_exif_tags().unwrap();
		for e in exifs {
			if meta.get_tag_string(&e).is_err() {
				continue;
			}

			let tag = meta.get_tag_string(&e).unwrap_or(String::new()).to_string();

			data.push(Data {
				tag: e.clone(),
				value: Some(if tag.len() > 80 && show_raw {
					if hex::decode(&tag).is_ok() {
						String::from_utf8(hex::decode(&tag).unwrap())
							.unwrap_or(truncate(tag.as_ref(), 40).to_owned() + "...")
					} else if try_string_of_bytes_to_string(&tag).is_ok() {
						truncate(try_string_of_bytes_to_string(&tag).unwrap().as_ref(), 40).to_owned() + "[r]"
					} else {
						truncate(tag.as_ref(), 40).to_owned() + "..."
					}
				} else {
					tag
				}),
			});
		}
	};

	if meta.has_iptc() {
		let iptcs = meta.get_iptc_tags().unwrap();
		for e in iptcs {
			if meta.get_tag_string(&e).is_err() {
				continue;
			}

			data.push(Data {
				tag: e.clone(),
				value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string()),
			});
		}
	};

	if meta.has_xmp() {
		let xmps = meta.get_xmp_tags().unwrap();
		for e in xmps {
			if meta.get_tag_string(&e).is_err() {
				continue;
			}

			data.push(Data {
				tag: e.clone(),
				value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string()),
			});
		}
	};

	let mut table = Table::new();
	table
		.set_header(vec!["Tag", "Value"])
		.set_header(vec![name])
		.load_preset(comfy_table::presets::UTF8_BORDERS_ONLY)
		//.apply_modifier(comfy_table::modifiers::UTF8_SOLID_INNER_BORDERS)
		.set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
		.set_table_width(
			termsize::get()
				.unwrap_or(termsize::Size {
					rows: 50,
					cols: 150,
				})
				.cols,
		);

	for entry in data.clone() {
		table.add_row(vec![entry.tag, entry.value.unwrap_or(String::new())]);
	}

	println!("{table}");
}

fn try_string_of_bytes_to_string(s: &String) -> Result<String, u8> {
	let white_space_seperated = s
		.split_whitespace()
		.collect::<Vec<&str>>();

	let sep: Vec<u16> = white_space_seperated.iter()
	.map(|f|
		if f.parse::<u16>().is_err()
		{
			0u16
		}
		else
		{f.parse::<u16>().unwrap()}
	)
	.collect();

	let x = decode_utf16(sep).map(|f| f.unwrap()).collect();

	Ok(x)
}

fn convert_folder_input_into_files_within(input: Vec<String>) -> Vec<String> {
	let mut x: Vec<String> = Vec::new();
	for entry in input {
		if PathBuf::from(&entry).is_file() {
			x.push(entry)
		} else {
			let paths = fs::read_dir(entry).unwrap();

			for path in paths {
				x.push(path.unwrap().path().into_os_string().into_string().unwrap());
			}
		};
	}

	x
}

#[derive(std::clone::Clone)]
struct Data {
	tag: String,
	value: Option<String>,
}

// https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters
fn truncate(s: &str, max_chars: usize) -> &str {
	match s.char_indices().nth(max_chars) {
		None => &s,
		Some((idx, _)) => &s[..idx],
	}
}
