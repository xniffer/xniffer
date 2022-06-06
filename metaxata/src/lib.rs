pub mod data;
pub mod provider;
pub mod provider_id3;
pub mod provider_kamadak;
pub mod provider_system;
pub mod test;
pub mod value;

use std::path::PathBuf;

use data::Data;

pub fn get_tags(file: &PathBuf) -> Vec<Data> {
	let mut data: Vec<Data> = Vec::new();
	data.append(&mut provider_kamadak::get_tags(file));
	data
}
