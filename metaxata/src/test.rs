#[cfg(test)]
mod tests {
	use crate::{provider::*, value::Value};

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
}
