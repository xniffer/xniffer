use std::{fs::File, io::BufReader, path::PathBuf};

use exif::{Rational, SRational};

use crate::{data::Data, provider::Provider, value::Value};

/// `kamadak-exif` provider
///
/// Prefix: `Kamadak`
///
/// Example:
/// - `Kamadak.ImageWidth`: `1920`
/// - `Kamadak.ImageLength`: `1080`

pub fn get_tags(file: &PathBuf) -> Option<Vec<Data>> {
	let file = File::open(file.canonicalize().unwrap()).unwrap();
	let exif = exif::Reader::new().read_from_container(&mut BufReader::new(&file));

	if exif.is_err() {
		return None;
	}

	Some(
		exif.unwrap()
			.fields()
			.map(|f| {
				(
					match &f.value {
						exif::Value::Byte(r) => Value::Raw(r.to_vec()),
						exif::Value::Ascii(s) => Value::String(vec_vec_8_to_string(s)),
						exif::Value::Short(v) => Value::Integer(vec_u16_to_i64(v)),
						exif::Value::Long(v) => Value::Integer(vec_u32_to_i64(v)),
						exif::Value::Rational(v) => Value::Ration(vec_rat_to_f64x2(v)),
						exif::Value::SByte(_) => Value::TODO,
						exif::Value::Undefined(v, _) => Value::Raw(v.to_vec()),
						exif::Value::SShort(v) => Value::Integer(vec_i16_to_i64(v)),
						exif::Value::SLong(v) => Value::Integer(vec_i32_to_i64(v)),
						exif::Value::SRational(v) => Value::Ration(vec_srat_to_f64x2(v)),
						exif::Value::Float(v) => Value::Number(vec_f32_to_f64(v)),
						exif::Value::Double(v) => Value::Number(vec_f64_to_f64(v)),
						exif::Value::Unknown(_, _, _) => Value::Error(
							"Raw value within kamadak-exif, reading is a planned feature."
								.to_string(),
						),
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

fn vec_u32_to_i64(val: &Vec<u32>) -> i64 {
	let mut x = 0i64;
	val.iter().for_each(|v| {
		x += *v as i64;
	});

	x
}

fn vec_u16_to_i64(val: &Vec<u16>) -> i64 {
	let mut x = 0i64;
	val.iter().for_each(|v| {
		x += *v as i64;
	});

	x
}

fn vec_rat_to_f64x2(val: &Vec<Rational>) -> [f64; 2] {
	[
		val.first().unwrap().num as f64,
		val.first().unwrap().denom as f64,
	]
}

fn vec_srat_to_f64x2(val: &Vec<SRational>) -> [f64; 2] {
	[
		val.first().unwrap().num as f64,
		val.first().unwrap().denom as f64,
	]
}

fn vec_f32_to_f64(val: &Vec<f32>) -> f64 {
	let mut x = 0f64;
	val.iter().for_each(|v| {
		x += *v as f64;
	});

	x
}

fn vec_f64_to_f64(val: &Vec<f64>) -> f64 {
	let mut x = 0f64;
	val.iter().for_each(|v| {
		x += *v as f64;
	});

	x
}

fn vec_i16_to_i64(val: &Vec<i16>) -> i64 {
	let mut x = 0i64;
	val.iter().for_each(|v| {
		x += *v as i64;
	});

	x
}

fn vec_i32_to_i64(val: &Vec<i32>) -> i64 {
	let mut x = 0i64;
	val.iter().for_each(|v| {
		x += *v as i64;
	});

	x
}
