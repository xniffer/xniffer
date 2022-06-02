use std::path::PathBuf;

use crate::value::Value;

/// `kamadak-exif` provider
///
/// Prefix: `Kamadak`
///
/// Example:

pub fn list_tags(file: &PathBuf) -> Vec<String> {
    let file = std::fs::File::open(file).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    exif.fields().map(|f| f.tag.to_string()).collect()
}

pub fn get_tag(file: PathBuf, tag: String) -> Value {
    let file = std::fs::File::open(file).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    exif.fields().map(|f|
        match f.value
        {
            exif::Value::Byte(r) => Value::Raw(r),
            exif::Value::Ascii(s) => todo!(),
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
        }
    ).collect()
}
