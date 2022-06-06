use std::{path::PathBuf, fs::File, io::BufReader};

use crate::{data::Data, provider::Provider, value::Value};

/// `kamadak-exif` provider
///
/// Prefix: `Kamadak`
///
/// Example:

pub fn get_tags(file: &PathBuf) -> Option<Vec<Data>> {
    let file = File::open(file.as_path()).unwrap();
    let exif = exif::Reader::new().read_from_container(
        &mut BufReader::new(&file));

	/*if exif.is_err()
	{
		return None;
	}*/

	Some(
		exif.unwrap().fields()
			.map(|f| {
				(
					match &f.value {
						exif::Value::Byte(r) => Value::Raw(r.to_vec()),
						exif::Value::Ascii(s) => Value::String(vec_vec_8_to_string(s)),
						exif::Value::Short(_) => todo!(),
						exif::Value::Long(_) => todo!(),
						exif::Value::Rational(_) => todo!(),
						exif::Value::SByte(_) => todo!(),
						exif::Value::Undefined(_, _) => todo!(),
						exif::Value::SShort(_) => todo!(),
						exif::Value::SLong(_) => todo!(),
						exif::Value::SRational(_) => todo!(),
						exif::Value::Float(_) => todo!(),
						exif::Value::Double(_) => todo!(),
						exif::Value::Unknown(_, _, _) => Value::Error("Raw value".to_string()),
					},
					f.tag,
				)
			})
			.map(|f| Data {
				tag: f.1.to_string(),
				value: f.0,
				provider: Provider::Kamadak,
			})
			.collect(),
	)
}

fn vec_vec_8_to_string(val: &Vec<Vec<u8>>) -> String {
	val.iter()
		.map(|f| std::str::from_utf8(f).unwrap_or(""))
		.collect::<Vec<&str>>()
		.join("")
}
