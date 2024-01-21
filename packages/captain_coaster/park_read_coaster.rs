use serde::{de::Error, Deserialize, Deserializer};

/// Park
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ParkReadCoaster {
	#[serde(rename(deserialize = "@id"))]
	#[serde(deserialize_with = "deserialize_id")]
	pub id: u32,
	pub country: Option<crate::country_read_coaster::CountryReadCoaster>,
	pub name: Option<String>,
}

fn deserialize_id<'de, D>(id: D) -> Result<u32, D::Error>
where
	D: Deserializer<'de>,
{
	let id = String::deserialize(id)?
		.strip_prefix("/api/parks/")
		.ok_or(Error::custom("could not remove `/api/parks/` prefix"))?
		.parse::<u32>()
		.map_err(serde::de::Error::custom)?;

	return Ok(id);
}

impl ParkReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ParkReadCoasterBuilder {
		ParkReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<ParkReadCoaster> for ParkReadCoasterBuilder {
	fn into(self) -> ParkReadCoaster {
		self.body
	}
}

/// Builder for [`ParkReadCoaster`](./struct.ParkReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct ParkReadCoasterBuilder {
	body: self::ParkReadCoaster,
}

impl ParkReadCoasterBuilder {
	#[inline]
	pub fn country(mut self, value: crate::country_read_coaster::CountryReadCoaster) -> Self {
		self.body.country = Some(value.into());
		self
	}

	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
