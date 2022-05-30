pub mod data;
pub mod provider;
pub mod test;
pub mod value;
pub mod provider_system;

use std::path::PathBuf;

/// Get all the tags in a file
pub fn get_tags(file: PathBuf) -> Vec<String>{
	provider_system::get_tags(file)
}
