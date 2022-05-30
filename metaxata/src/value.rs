#[allow(dead_code)]

/// A type representing a value with a type
#[derive(Clone, PartialEq)]
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
}
