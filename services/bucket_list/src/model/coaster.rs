use schemars::JsonSchema;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::model::{manufacturer::Manufacturer, park::Park};

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
pub struct Coaster {
	pub id: u32,
	pub name: String,
	pub speed: Option<u32>,
	pub height: Option<u32>,
	pub inversions: Option<u32>,
	pub manufacturer: Option<Manufacturer>,
	pub park: Option<Park>,
	pub image: Option<String>,
}
