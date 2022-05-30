#[allow(dead_code)]

use crate::provider::Provider;
use crate::value::Value;

#[derive(Clone)]
pub struct Data {
	pub tag: String,
	pub value: Value,
	pub provider: Provider,
}
