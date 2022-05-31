use std::{path::PathBuf, time::SystemTime};

/// `System` provider
/// 
/// Prefix: `System`
/// 
/// Example:
/// - System.TimeCreated
/// - System.TimeAccessed
/// - System.TimeModified

pub fn list_tags(file: PathBuf) -> Vec<String> {
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

pub fn get_tag(file: PathBuf, tag: String) -> String
{
	match &tag as &str {
		"System.TimeCreated" => file.metadata().unwrap().created().unwrap_or(SystemTime::UNIX_EPOCH).elapsed().unwrap().as_secs_f64().to_string(),
	    _ => "Invalid tag, please report this as a bug".to_string(),
	}
}
