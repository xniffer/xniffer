extern crate clap;
extern crate comfy_table;
extern crate rayon;

use clap::{Arg, Command};
use colored::Colorize;
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
		.arg(
			Arg::new("PATHS")
				.required(true)
				.help("Specify paths")
				.takes_value(true)
				.multiple_values(true),
		)
		.arg(
			Arg::new("RAW")
				.help("show raw data")
				.short('r')
				.long("raw")
				.takes_value(false),
		)
		.after_help("https://github.com/3top1a/xniffer")
		.get_matches();

	let files: Vec<String> = convert_folder_input_into_files_within(
		matches.values_of_t("PATHS").unwrap_or_else(|e| e.exit()),
	);

	// Logic
	files
		.par_iter()
		.for_each(|x| parse(x, matches.is_present("RAW")));

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

	// Exif tags
	// This IS ugly, but .append is a mutating method and I don't know anything better
	let tags: Vec<String> = {
		meta.get_exif_tags()
			.unwrap()
			.into_iter()
			.chain(meta.get_iptc_tags().unwrap().into_iter())
			.into_iter()
			.chain(meta.get_xmp_tags().unwrap().into_iter())
			.collect()
	};

	let data: Vec<Data> = tags
		.iter()
		.map(|f| Data {
			tag: f.to_owned(),
			value: Some(process_tag_value(
				meta.get_tag_string(&f)
					.unwrap_or("Error!".red().to_string()),
				show_raw,
			)),
		})
		.collect();

	// TODO Refactor to imutability
	// ? Is it even possible?
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
		table.add_row(vec![
			entry.tag.green().to_string(),
			entry.value.unwrap_or(String::new()),
		]);
	}

	println!("{table}");
}

fn process_tag_value(value: String, show_raw: bool) -> String {
	if value.len() > 80 && !show_raw {
		if hex::decode(&value).is_ok() {
			String::from_utf8(hex::decode(&value).unwrap())
				.unwrap_or(truncate(value.as_ref(), 40).to_owned() + &"...".yellow())
		} else if try_string_of_bytes_to_string(&value).is_ok() {
			truncate(try_string_of_bytes_to_string(&value).unwrap().as_ref(), 40).to_owned()
				+ &"[r]".yellow()
		} else {
			truncate(value.as_ref(), 40).to_owned() + &"...".yellow()
		}
	} else {
		value
	}
}

// Takes a string like `85 74 69`
// And outputs `abc`
fn try_string_of_bytes_to_string(s: &String) -> Result<String, u8> {
	let white_space_seperated = s.split_whitespace().collect::<Vec<&str>>();

	let sep: Vec<u16> = white_space_seperated
		.iter()
		.map(|f| {
			if f.parse::<u16>().is_err() {
				0u16
			} else {
				f.parse::<u16>().unwrap()
			}
		})
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
