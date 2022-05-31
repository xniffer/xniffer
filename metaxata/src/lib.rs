pub mod data;
pub mod provider;
pub mod provider_system;
pub mod test;
pub mod value;

use std::path::PathBuf;

/// Get all the tags in a file
pub fn list_tags(file: PathBuf) -> Vec<String> {
	provider_system::list_tags(file)
}

pub fn get_tag(file: PathBuf, tag: String) -> String {
	provider_system::get_tag(file, tag)
}
