extern crate comfy_table;
extern crate rayon;

use comfy_table::Table;
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
	println!("--- xniffer v{} ---", std::env!("CARGO_PKG_VERSION"));
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		panic!("No files provided!");
	}

	let files: Vec<String> = convert_folder_input_into_files_within(Vec::from(&args[1..]));

	// Logic
	files.par_iter().for_each(|x| parse(x));

	Ok(())
}

fn parse(name: &String) {
	// Try
	let metadata = rexiv2::Metadata::new_from_path(name);
	if metadata.is_err()
	{
		panic!("{} Could not be parsed.", name);
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
				value: Some(if tag.len() > 80 {
					truncate ( tag.as_ref(), 40).to_owned() + "..."
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
		.load_preset(comfy_table::presets::UTF8_FULL)
		.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
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

fn convert_folder_input_into_files_within(input: Vec<String>) -> Vec<String> {
	let mut x: Vec<String> = Vec::new();
	for entry in input {
		if PathBuf::from(&entry).is_file() {
			x.push(entry)
		} else {
			// TODO
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
