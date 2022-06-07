pub mod data;
pub mod provider;
pub mod provider_id3;
pub mod provider_kamadak;
pub mod provider_system;
pub mod test;
pub mod value;

use std::path::PathBuf;

use data::Data;

const UNDESURED_BEHAVIOUR_NOTE: &str =
	"Please report this to the metaxata's github page, with the picture for debugging purposes. ";

pub fn get_tags(file: &PathBuf) -> Option<Vec<Data>> {
	if file.is_dir() {
		return None;
	}

	let mut data: Vec<Data> = Vec::new();
	data.append(&mut provider_system::get_tags(file));
	data.append(&mut provider_kamadak::get_tags(file).unwrap_or(vec![]));
	Some(data)
}
