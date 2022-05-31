pub mod data;
pub mod provider;
pub mod provider_system;
pub mod test;
pub mod value;

use std::path::PathBuf;

/// Get all the tags in a file
pub fn get_tags(file: PathBuf) -> Vec<String> {
	return provider_system::get_tags(file)
}
