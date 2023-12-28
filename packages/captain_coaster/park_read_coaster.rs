/// Park
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ParkReadCoaster {
	pub country: Option<crate::country_read_coaster::CountryReadCoaster>,
	pub name: Option<String>,
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
