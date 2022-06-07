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
			Value::Raw(_v) => write!(f, "RAW DATA DETECTED, THIS IS A PLANNED FEATURE"), // TODO Display raw data
			Value::Error(v) => write!(f, "Error! {}", v),
			Value::TODO => write!(f, "TODO spotted, please report this to the projects github")
		}
	}
}
