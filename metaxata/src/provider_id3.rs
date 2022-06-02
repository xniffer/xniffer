use std::path::PathBuf;

use id3::TagLike;

use crate::value::Value;

extern crate id3;

/// `ID3` provider
///
/// Prefix: `id3`
///
/// Example:
/// - id3.Year
/// - id3.Title

pub fn list_tags(file: &PathBuf) -> Vec<String> {
	// Check for error
	if id3::Tag::read_from_path(file).is_err() {
		return vec![];
	}

	let mut data: Vec<String> = Vec::new();

	let tag = id3::Tag::read_from_path(file).unwrap();
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

pub fn get_tag(file: PathBuf, tag: String) -> Value {
	match &tag as &str {
		"System.TimeCreated" => Value::Time(
			file.metadata()
				.unwrap()
				.created()
				.unwrap()
				.elapsed()
				.unwrap()
				.as_secs(),
		),
		"System.TimeAccessed" => Value::Time(
			file.metadata()
				.unwrap()
				.accessed()
				.unwrap()
				.elapsed()
				.unwrap()
				.as_secs(),
		),
		"System.TimeModified" => Value::Time(
			file.metadata()
				.unwrap()
				.modified()
				.unwrap()
				.elapsed()
				.unwrap()
				.as_secs(),
		),
		_ => Value::Error("Invalid tag, please report this as a bug".to_string()),
	}
}
