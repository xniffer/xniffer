use std::fmt::Display;

#[allow(dead_code)]

/// A type representing a value with a type
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
	/// A simple string
	///
	/// Example
	/// - `Exif.Image.Make`: `NIKON`
	String(String),
	/// 64 Bit signed integer
	///
	/// Example
	/// - `Exif.Photo.ExposureMode`: `0`
	Integer(i64),
	/// 64 Bit signed float
	Number(f64),
	/// Ration, represented as two values.
	///
	/// To get the result, use `[0] / [1]`.
	Ration([f64; 2]),
	/// Time represented in Unix Epoch
	///
	/// Example:
	/// - `Exif.Sample.TimeTaken`: `2,147,483,649`
	Time(u64),
	/// Raw data that cannot be parsed, such as proprietary data
	///
	/// Example:
	/// - `Exif.Nikon3.0x00ae`: `0 0 2 0 0 0 0 0 0 0 0 0`
	Raw(Vec<u8>),
	/// Error
	Error(String),
	/// TODO, Mostly for developing
	TODO,
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::String(v) => write!(f, "{}", v),
			Value::Integer(v) => write!(f, "{}", v),
			Value::Number(v) => write!(f, "{}", v),
			Value::Time(v) => write!(f, "Unix: {}", v),
			Value::Raw(v) => write!(f, "{}", try_vec_8_to_string(v)), // TODO Display raw data
			Value::Error(v) => write!(f, "Error! {}{}", crate::UNDESURED_BEHAVIOUR_NOTE, v),
			Value::TODO => write!(f, "TODO value type"),
			Value::Ration(v) => write!(f, "{}/{}, {}", v[0], v[1], v[0] / v[1]),
		}
	}
}

fn try_vec_8_to_string(val: &Vec<u8>) -> String
{
	if std::str::from_utf8(val).is_ok()
	{
		std::str::from_utf8(val).unwrap().to_string()
	}
	else
	{
		// If all else fails, print the numbers out in hex
		val.iter().map(|f:&u8| format!("{:X}", f).to_string() ).collect::<Vec<String>>().join("")
	}
}

impl Value {
	/// Returns the name of the type, e.g. `String`, `Time`
	pub fn name_to_string(&self) -> &str {
		match self {
			Value::String(_) => "String",
			Value::Integer(_) => "Integer",
			Value::Number(_) => "Number",
			Value::Ration(_) => "Ration",
			Value::Time(_) => "Time",
			Value::Raw(_) => "Raw",
			Value::Error(_) => "Error",
			Value::TODO => "TODO",
		}
	}
}
