use std::{path::PathBuf, time::SystemTime};

use crate::value::Value;

/// `System` provider
///
/// Prefix: `System`
///
/// Example:
/// - System.TimeCreated
/// - System.TimeAccessed
/// - System.TimeModified

pub fn list_tags(file: &PathBuf) -> Vec<String> {
	// Check for error
	if file.metadata().is_err() {
		return vec![];
	}

	let mut data: Vec<String> = Vec::new();

	let met = file.metadata().unwrap();
	if met.created().is_ok() {
		data.push("System.TimeCreated".to_string())
	};
	if met.accessed().is_ok() {
		data.push("System.TimeAccessed".to_string())
	};
	if met.modified().is_ok() {
		data.push("System.TimeModified".to_string())
	};

	data
}

pub fn get_tag(file: PathBuf, tag: String) -> Value {
	match &tag as &str {
		"System.TimeCreated" => Value::Time(
			file.metadata()
				.unwrap()
				.created()
				.unwrap()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs()
		),
		"System.TimeAccessed" => Value::Time(
			file.metadata()
				.unwrap()
				.accessed()
				.unwrap()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs()
		),
		"System.TimeModified" => Value::Time(
			file.metadata()
				.unwrap()
				.modified()
				.unwrap()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs()
		),
		_ => Value::Error("Invalid tag, please report this as a bug".to_string()),
	}
}
