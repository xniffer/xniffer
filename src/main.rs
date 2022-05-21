extern crate clap;
extern crate comfy_table;
extern crate rayon;

use clap::{Arg, Command};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod cli;
mod gui;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	println!("⎯⎯⎯ xniffer v{} ⎯⎯⎯", std::env!("CARGO_PKG_VERSION"));

	let matches = Command::new("xniffer")
		.version(CARGO_PKG_VERSION)
		.author(CARGO_PKG_AUTHORS)
		.about("A simple exif sniffer written in Rust")
		.arg(
			Arg::new("PATHS")
				.help("Specify paths")
				.takes_value(true)
				.multiple_values(true),
		)
		.arg(
			Arg::new("RAW")
				.help("show raw data, only for CLI")
				.short('r')
				.long("raw")
				.takes_value(false),
		)
		.arg(
			Arg::new("ASCII")
				.help("show table in ascii instead of unicode, only for CLI")
				.short('a')
				.long("ascii")
				.takes_value(false),
		)
		.arg(
			Arg::new("NOTABLE")
				.help("don't format the data to a table, only for CLI")
				.short('n')
				.long("notable")
				.takes_value(false),
		)
		.arg(
			Arg::new("CLI")
				.help("use CLI")
				.short('c')
				.long("cli")
				.takes_value(false),
		)
		.after_help("https://github.com/3top1a/xniffer")
		.get_matches();

	// Get list of all files
	let files: Vec<String> = if matches.is_present("PATHS") {
		convert_folder_input_into_files_within(
			matches.values_of_t("PATHS").unwrap_or_else(|e| e.exit()),
		)
	} else {
		vec![]
	};

	// Logic
	if !matches.is_present("CLI") {
		gui::display(files)
	} else {
		files.par_iter().for_each(|file| {
			cli::display(
				file.to_string(),
				parse(file),
				matches.is_present("RAW"),
				matches.is_present("ASCII"),
				matches.is_present("NOTABLE"),
			)
		})
	}

	Ok(())
}

fn parse(name: &String) -> Option<Vec<Data>> {
	// Try
	let metadata = rexiv2::Metadata::new_from_path(name);
	if metadata.is_err() {
		println!("{} Could not be parsed.", name);
		return None;
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
			value: meta.get_tag_string(&f).unwrap_or("Error!".to_string()),
		})
		.collect();

	Some(data)
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

#[derive(Debug)]
pub struct Data {
	tag: String,
	value: String,
	//value: DataType<String>,
}

impl std::fmt::Display for Data {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}, {})", self.tag, self.value)
	}
}

impl Clone for Data {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			tag: self.tag.clone(),
			value: self.value.clone(),
		}
	}
}

// TODO Data type
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum DataType<I> {
	String(I),
	Number(I),
	Ration(I),
	Raw(I),
	GPS([i64; 2]),
}
