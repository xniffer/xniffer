use std::path::PathBuf;

use crate::{data::Data, provider::Provider, value::Value};

/// `kamadak-exif` provider
///
/// Prefix: `Kamadak`
///
/// Example:

pub fn get_tags(file: &PathBuf) -> Vec<Data> {
	let file = std::fs::File::open(file).unwrap();
	let mut bufreader = std::io::BufReader::new(&file);
	let exifreader = exif::Reader::new();
	let exif = exifreader.read_from_container(&mut bufreader).unwrap();
	exif.fields()
		.map(|f| {
			(
				match &f.value {
					exif::Value::Byte(r) => Value::Raw(r.to_vec()),
					exif::Value::Ascii(s) => Value::Error("TODO".to_string()),
					exif::Value::Short(i) => todo!(),
					exif::Value::Long(i) => todo!(),
					exif::Value::Rational(_) => todo!(),
					exif::Value::SByte(_) => todo!(),
					exif::Value::Undefined(_, _) => todo!(),
					exif::Value::SShort(_) => todo!(),
					exif::Value::SLong(_) => todo!(),
					exif::Value::SRational(_) => todo!(),
					exif::Value::Float(f) => todo!(),
					exif::Value::Double(f) => todo!(),
					exif::Value::Unknown(_, _, _) => todo!(),
				},
				f.tag,
			)
		})
		.map(|f| Data {
			tag: f.1.to_string(),
			value: f.0,
			provider: Provider::Kamadak,
		})
		.collect()
}
