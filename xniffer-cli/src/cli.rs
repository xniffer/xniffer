use comfy_table::*;

use metaxata::data::Data;

pub fn display(name: String, data: Vec<Data>, show_ascii: bool, notable: bool) {
	// Create table
	let preset = if notable {
		comfy_table::presets::NOTHING
	} else if show_ascii {
		comfy_table::presets::ASCII_BORDERS_ONLY_CONDENSED
	} else {
		comfy_table::presets::UTF8_BORDERS_ONLY
	};

	let mut table = Table::new();
	table
		.set_header(vec!["Tag", "Value"])
		.set_header(vec![Cell::new(name).add_attribute(Attribute::Bold)])
		.load_preset(preset)
		.set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
		.set_table_width(
			termsize::get()
				.unwrap_or(termsize::Size {
					rows: 50,
					cols: 150,
				})
				.cols,
		);

	for entry in data {
		table.add_row(vec![
			Cell::new(format!(
				"[{}] {}/{}",
				truncate(entry.value.name_to_string(), 3),
				entry.provider.to_string(),
				&entry.tag
			))
			.fg(Color::Green)
			.set_alignment(CellAlignment::Left)
			.add_attribute(Attribute::Italic),
			Cell::new(&entry.value),
		]);
	}

	// Print
	println!("{table}");
}

// https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters
fn truncate(s: &str, max_chars: usize) -> &str {
	match s.char_indices().nth(max_chars) {
		None => &s,
		Some((idx, _)) => &s[..idx],
	}
}
