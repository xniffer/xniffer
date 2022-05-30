#[cfg(test)]
mod tests {
	use crate::{data::Data, provider::Provider, value::Value};

	#[test]
	fn provider_eq() {
		let p1 = Provider::System;
		let p2 = Provider::Lofty;
		let p3 = Provider::System;

		assert!(p1 == p3);
		assert!(p1 != p2);
		assert!(p3 != p2);
	}

	#[test]
	fn value_eq() {
		let v1 = Value::String("Sample 1".to_string());
		let v2 = Value::Time(0);
		let v3 = Value::String("Sample 1".to_string());
		let v4 = Value::String("Sample 2".to_string());

		assert!(v1 == v3);
		assert!(v1 != v2);
		assert!(v1 != v4);
	}

	#[test]
	fn data_eq() {
		let d1 = Data {
			tag: "Exif.Example.Tag1".to_string(),
			value: Value::String("Example".to_string()),
			provider: Provider::System,
		};
		let d2 = Data {
			tag: "Exif.Data.Tag2".to_string(),
			value: Value::String("Really".to_string()),
			provider: Provider::System,
		};
		let d3 = Data {
			tag: "Exif.Example.Tag1".to_string(),
			value: Value::String("Example".to_string()),
			provider: Provider::System,
		};

		assert!(d1 == d3);
		assert!(d1 != d2);
		assert!(d3 != d2);

		assert!(d1.provider == d2.provider);
		assert!(d1.value == d3.value);
		assert!(d1.tag == d3.tag);
	}
}
