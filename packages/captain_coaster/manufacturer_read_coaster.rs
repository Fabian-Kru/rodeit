///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ManufacturerReadCoaster {
	pub name: Option<String>,
}

impl ManufacturerReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ManufacturerReadCoasterBuilder {
		ManufacturerReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<ManufacturerReadCoaster> for ManufacturerReadCoasterBuilder {
	fn into(self) -> ManufacturerReadCoaster {
		self.body
	}
}

/// Builder for [`ManufacturerReadCoaster`](./struct.ManufacturerReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct ManufacturerReadCoasterBuilder {
	body: self::ManufacturerReadCoaster,
}

impl ManufacturerReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
