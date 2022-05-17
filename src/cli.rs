use comfy_table::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{utils::process_tag_value, Data};

pub fn display(name: String, data: Option<Vec<Data>>) {
	if data.is_none() {
		println!("Error!");
		return;
	}

	// Process raw values
	let processed_data: Vec<Data> = data
		.to_owned()
		.unwrap()
		.par_iter()
		.map(|d| d.to_owned())
		// TODO Immutability
		.map(|mut f| process_tag_value(&mut f, false).to_owned())
		.collect();

	// Create table
	let mut table = Table::new();
	table
		.set_header(vec!["Tag", "Value"])
		.set_header(vec![Cell::new(name).add_attribute(Attribute::Bold)])
		.load_preset(comfy_table::presets::UTF8_BORDERS_ONLY)
		.set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
		.set_table_width(
			termsize::get()
				.unwrap_or(termsize::Size {
					rows: 50,
					cols: 150,
				})
				.cols,
		);

	for entry in processed_data {
		table.add_row(vec![
			Cell::new(&entry.tag)
				.fg(Color::Green)
				.add_attribute(Attribute::Italic),
			Cell::new(&entry.value),
		]);
	}

	// Print
	println!("{table}");
}
