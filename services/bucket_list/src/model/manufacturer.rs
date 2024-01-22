use schemars::JsonSchema;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
pub struct Manufacturer {
	pub name: String,
}
