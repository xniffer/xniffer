extern crate clap;
extern crate comfy_table;
extern crate rayon;

use clap::{Arg, Command};
use metaxata::data::Data;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod cli;

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
	files.par_iter().for_each(|file| {
		cli::display(
			file.to_string(),
			parse(file),
			matches.is_present("ASCII"),
			matches.is_present("NOTABLE"),
		)
	});

	Ok(())
}

fn parse(name: &String) -> Vec<Data> {
	let path = &PathBuf::from(name);

	// Exif tags
	let tags: Vec<Data> = metaxata::get_tags(path).unwrap();
	tags
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
