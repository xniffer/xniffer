pub mod data;
pub mod provider;
pub mod provider_system;
pub mod test;
pub mod value;

use std::path::PathBuf;

use data::Data;
use provider::Provider;

/// Get all the tags in a file
pub fn list_tags(file: &PathBuf) -> Vec<String> {
	provider_system::list_tags(&file)
}

pub fn get_tag(file: PathBuf, tag: String) -> Data {
	let (value, provider) = if tag.starts_with("") {
		(
			provider_system::get_tag(file, tag.clone()),
			Provider::System,
		)
	} else {
		(
			value::Value::Error("Unable to locate provider, please report this".to_string()),
			Provider::Unknown,
		)
	};

	Data {
		tag,
		value,
		provider,
	}
}
