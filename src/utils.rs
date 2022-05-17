use std::char::decode_utf16;

use crate::Data;

// https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters
fn truncate(s: &str, max_chars: usize) -> &str {
	match s.char_indices().nth(max_chars) {
		None => &s,
		Some((idx, _)) => &s[..idx],
	}
}

pub fn process_tag_value(value: &mut Data, show_raw: bool) -> &mut Data {
	if value.value.len() > 80 && !show_raw {
		if hex::decode(&value.value).is_ok() {
			value.value = String::from_utf8(hex::decode(&value.value).unwrap())
				.unwrap_or(truncate(value.value.as_ref(), 40).to_owned() + "...");
			value
		} else if try_string_of_bytes_to_string(&value.value).is_ok() {
			value.value = truncate(
				try_string_of_bytes_to_string(&value.value)
					.unwrap()
					.as_ref(),
				40,
			)
			.to_owned() + "[r]";
			value
		} else {
			value.value = truncate(value.value.as_ref(), 40).to_owned() + "...";
			value
		}
	} else {
		value
	}
}

// Takes a string like `85 74 69`
// And outputs `abc`
pub fn try_string_of_bytes_to_string(s: &String) -> Result<String, u8> {
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
