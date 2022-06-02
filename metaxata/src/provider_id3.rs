use std::path::{Path, PathBuf};

use other_id3::TagLike;

use crate::value::Value;

extern crate id3 as other_id3;

/// `ID3` provider
///
/// Prefix: `id3`
///
/// Example:
/// - id3.Year
/// - id3.Title

pub fn list_tags(file: &Path) -> Vec<String> {
	// Check for error
	if other_id3::Tag::read_from_path(file).is_err() {
		return vec![];
	}

	let mut data: Vec<String> = Vec::new();

	let tag = other_id3::Tag::read_from_path(file).unwrap();
	if tag.year().is_some() {
		data.push("id3.Year".to_string());
	}
	if tag.track().is_some() {
		data.push("id3.Track".to_string());
	}
	if tag.title().is_some() {
		data.push("id3.Title".to_string());
	}
	if tag.genre().is_some() {
		data.push("id3.Genre".to_string());
	}
	if tag.duration().is_some() {
		data.push("id3.Length".to_string());
	}
	if tag.disc().is_some() {
		data.push("id3.DiscNumber".to_string());
	}
	if tag.date_released().is_some() {
		data.push("id3.Released".to_string());
	}
	if tag.date_recorded().is_some() {
		data.push("id3.Recorded".to_string());
	}
	if tag.album().is_some() {
		data.push("id3.Album".to_string());
	}
	if tag.artist().is_some() {
		data.push("id3.Artist".to_string());
	}

	data
}

pub fn get_tag(file: PathBuf, tag_s: String) -> Value {
	let tag = other_id3::Tag::read_from_path(file).unwrap();

	match &tag_s as &str {
		"id3.Year" => Value::Integer(tag.year().unwrap() as i64),
		"id3.Track" => Value::Integer(tag.track().unwrap() as i64),
		"id3.Title" => Value::String(tag.title().unwrap().to_owned()),
		"id3.Genre" => Value::String(tag.genre().unwrap().to_owned()),
		"id3.Length" => Value::Integer(tag.duration().unwrap() as i64),
		"id3.DiscNumber" => Value::Integer(tag.disc().unwrap() as i64),
		"id3.Released" => Value::Time(timestamp_to_unix(tag.date_released().unwrap())),
		"id3.Recorded" => Value::Time(timestamp_to_unix(tag.date_recorded().unwrap())),
		"id3.Album" => Value::String(tag.album().unwrap().to_owned()),
		"id3.Artist" => Value::String(tag.artist().unwrap().to_owned()),
		_ => Value::Error("Invalid tag, please report this as a bug".to_string()),
	}
}

// TODO Finish
fn timestamp_to_unix(t: other_id3::Timestamp) -> u64 {
	(t.year as u64) * (31557600) // Years
}
