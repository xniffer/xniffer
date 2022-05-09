extern crate comfy_table;
extern crate rayon;

use comfy_table::Table;
use rayon::prelude::*;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
	println!("--- xniffer v{} ---", std::env!("CARGO_PKG_VERSION"));
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		panic!("No files provided!");
	}

	// Assuming the last argument is the file
	let files_input: Vec<PathBuf> = Vec::from(&args[1..])
		.into_par_iter()
		.map(|x| PathBuf::from(x))
		.collect();

	// Logic
	files_input.par_iter().for_each(|x| parse(x));

	Ok(())
}

fn parse(path: &PathBuf) {
	//println!("Parsing {}", &path.display());
	let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
	let mut data: Vec<Data> = Vec::new();

	// Exif tags
	if meta.has_exif() {
		let exifs = meta.get_exif_tags().unwrap();
		for e in exifs {
			data.push(Data {
				tag: e.clone(),
				value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string()),
			});
		}
	};

	if meta.has_iptc() {
		let iptcs = meta.get_iptc_tags().unwrap();
		for e in iptcs {
			data.push(Data {
				tag: e.clone(),
				value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string()),
			});
		}
	};

	if meta.has_xmp() {
		let xmps = meta.get_xmp_tags().unwrap();
		for e in xmps {
			data.push(Data {
				tag: e.clone(),
				value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string()),
			});
		}
	};

	let mut table = Table::new();
	table
		.set_header(vec!["Tag", "Value"])
		.load_preset(comfy_table::presets::UTF8_FULL)
		.apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
		.set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
		.set_table_width(
			termsize::get()
				.unwrap_or(termsize::Size { rows: 50, cols: 150 })
				.cols,
		);

	for entry in data.clone() {
		table.add_row(vec![entry.tag, entry.value.unwrap_or(String::new())]);
	}

	println!("{}\n{table}", path.display());
}

#[derive(std::clone::Clone)]
struct Data {
	tag: String,
	value: Option<String>,
}
