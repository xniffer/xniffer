use std::{path::Path, time::SystemTime};

use crate::{data::Data, provider::Provider, value::Value};

/// `System` provider
///
/// Prefix: `System`
///
/// Example:
/// - System.TimeCreated
/// - System.TimeAccessed
/// - System.TimeModified

pub fn get_tags(file: &Path) -> Vec<Data> {
	// Check for error
	if file.metadata().is_err() {
		return vec![];
	}

	let mut data: Vec<Data> = Vec::new();

	let met = file.metadata().unwrap();
	if met.created().is_ok() {
		data.push(Data {
			tag: "System.TimeCreated".to_string(),
			value: Value::Time(
				file.metadata()
					.unwrap()
					.created()
					.unwrap()
					.duration_since(SystemTime::UNIX_EPOCH)
					.unwrap()
					.as_secs(),
			),
			provider: Provider::System,
		})
	};
	if met.accessed().is_ok() {
		data.push(Data {
			tag: "System.TimeAccessed".to_string(),
			value: Value::Time(
				file.metadata()
					.unwrap()
					.created()
					.unwrap()
					.duration_since(SystemTime::UNIX_EPOCH)
					.unwrap()
					.as_secs(),
			),
			provider: Provider::System,
		})
	};
	if met.modified().is_ok() {
		data.push(Data {
			tag: "System.TimeModified".to_string(),
			value: Value::Time(
				file.metadata()
					.unwrap()
					.created()
					.unwrap()
					.duration_since(SystemTime::UNIX_EPOCH)
					.unwrap()
					.as_secs(),
			),
			provider: Provider::System,
		})
	};

	data
}

pub fn get_tag(file: &Path, tag: String) -> Value {
	match &tag as &str {
		"System.TimeCreated" => Value::Time(
			file.metadata()
				.unwrap()
				.created()
				.unwrap()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs(),
		),
		"System.TimeAccessed" => Value::Time(
			file.metadata()
				.unwrap()
				.accessed()
				.unwrap()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs(),
		),
		"System.TimeModified" => Value::Time(
			file.metadata()
				.unwrap()
				.modified()
				.unwrap()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs(),
		),
		_ => Value::Error("Invalid tag, please report this as a bug".to_string()),
	}
}
