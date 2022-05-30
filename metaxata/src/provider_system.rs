use std::{fs::Metadata, path::PathBuf};

pub fn get_tags(file: PathBuf) -> Vec<String> {
	if file.metadata().is_err()
	{
		return vec!()
	}

	let mut data: Vec<String> = Vec::new();

	let met = file.metadata().unwrap();
	if met.created().is_ok()
	{
		data.push("System.TimeCreated".to_string())
	};
	if met.accessed().is_ok()
	{
		data.push("System.TimeAccessed".to_string())
	};
	if met.modified().is_ok()
	{
		data.push("System.TimeModified".to_string())
	};

	data
}
