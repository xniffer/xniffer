extern crate rayon;
extern crate comfy_table;

use comfy_table::Table;use rayon::prelude::*;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
	println!("--- R-liv v{} ---", std::env!("CARGO_PKG_VERSION"));
	let args: Vec<String> = std::env::args().collect();

	if args.len() < 2 {
		panic!("No files provided!");
	}

	// Assuming the last argument is the file
	// TODO multiple files
	let files: Vec<PathBuf> = vec![PathBuf::from(&args[args.len() - 1])];

	// Logic
	let data: Vec<Vec<Data>> = files.par_iter().map(|x| parse(x).unwrap()).collect();

	let mut table = Table::new();
	table
		.set_header(vec!["Tag", "Value"])
	    .load_preset(comfy_table::presets::UTF8_FULL)
        .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_table_width(termsize::get().unwrap().cols);

	for file in data {
		for entry in file {
			table.add_row(vec![
				entry.tag,
				entry.value.unwrap_or(String::new()),
			]);
		}
	}

    println!("{table}");

	Ok(())
}

fn parse(path: &PathBuf) -> Result<Vec<Data>, String> {
	println!("Parsing {}", &path.display());

	/*let file = std::fs::File::open(path).unwrap();
	let mut bufreader = std::io::BufReader::new(&file);
	let exifreader = exif::Reader::new();
	let exif = exifreader.read_from_container(&mut bufreader).unwrap();
	for f in exif.fields() {
		println!("{} {} {}",
				 f.tag, f.ifd_num, f.display_value().with_unit(&exif));
		Data {
			tag: f.tag,
			ifd: f.ifd_num,
			value: Some(f.to_owned()),
		};
	}*/

	let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
	let mut data: Vec<Data> = Vec::new();

	// Exif tags
	if meta.has_exif() {
		let exifs = meta.get_exif_tags().unwrap();
		for e in exifs {
			data.push(
				Data {
					tag: e.clone(),
					value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string())
				}
			);
		};
	};

	if meta.has_iptc() {
		let iptcs = meta.get_iptc_tags().unwrap();
		for e in iptcs {
			data.push(
				Data {
					tag: e.clone(),
					value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string())
				}
			);
		};
	};

	if meta.has_xmp() {
		let xmps = meta.get_xmp_tags().unwrap();
		for e in xmps {
			data.push(
				Data {
					tag: e.clone(),
					value: Some(meta.get_tag_string(&e).unwrap_or(String::new()).to_string())
				}
			);
		};
	};

	Ok(data)
}

#[derive(std::clone::Clone)]
struct Data {
	tag: String,
	value: Option<String>,
}
