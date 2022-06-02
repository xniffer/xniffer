pub mod data;
pub mod provider;
pub mod provider_system;
pub mod test;
pub mod value;
//pub mod provider_kamadak;
pub mod provider_id3;

use std::path::PathBuf;

use data::Data;
use provider::Provider;

/// Get all the tags in a file
pub fn list_tags(file: &PathBuf) -> Vec<String> {
	let mut r: Vec<String> = Vec::new();
	r.append(&mut provider_system::list_tags(&file));
	//	r.append(&mut provider_kamadak::list_tags(&file));
	r
}

pub fn get_tag(file: PathBuf, tag: String) -> Data {
	let (value, provider) = if tag.starts_with("System") {
		(
			provider_system::get_tag(file, tag.clone()),
			Provider::System,
		)
	} else {
		(
			value::Value::Error(
				"Unable to locate provider, please report this at the GitHub repository"
					.to_string(),
			),
			Provider::Unknown,
		)
	};

	Data {
		tag,
		value,
		provider,
	}
}
