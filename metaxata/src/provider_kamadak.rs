use std::{path::PathBuf, fs::File, io::BufReader};

use crate::{data::Data, provider::Provider, value::Value};

/// `kamadak-exif` provider
///
/// Prefix: `Kamadak`
///
/// Example:
/// - `Kamadak.ImageWidth`: `1920`

pub fn get_tags(file: &PathBuf) -> Option<Vec<Data>> {
    let file = File::open(file.canonicalize().unwrap()).unwrap();
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
						exif::Value::Short(v) => Value::Integer(vec_u16_to_u64(v)),
						exif::Value::Long(v) => Value::Integer(vec_u32_to_u64(v)),
						exif::Value::Rational(_) => Value::TODO,
						exif::Value::SByte(_) => Value::TODO,
						exif::Value::Undefined(_, _) => Value::TODO,
						exif::Value::SShort(_) => Value::TODO,
						exif::Value::SLong(_) => Value::TODO,
						exif::Value::SRational(_) => Value::TODO,
						exif::Value::Float(_) => Value::TODO,
						exif::Value::Double(_) => Value::TODO,
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

// Helper functions for conversions

fn vec_vec_8_to_string(val: &Vec<Vec<u8>>) -> String {
	val.iter()
		.map(|f| std::str::from_utf8(f).unwrap_or(""))
		.collect::<Vec<&str>>()
		.join("")
}

fn vec_u32_to_u64 (val: &Vec<u32> ) -> i64
{
	let mut x = 0i64;
	val.iter().for_each ( |v| {
		x += *v as i64;
	});

	x
}

fn vec_u16_to_u64 (val: &Vec<u16> ) -> i64
{
	let mut x = 0i64;
	val.iter().for_each ( |v| {
		x += *v as i64;
	});

	x
}
