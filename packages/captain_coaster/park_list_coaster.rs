/// Park
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ParkListCoaster {
	pub name: Option<String>,
}

impl ParkListCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ParkListCoasterBuilder {
		ParkListCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<ParkListCoaster> for ParkListCoasterBuilder {
	fn into(self) -> ParkListCoaster {
		self.body
	}
}

/// Builder for [`ParkListCoaster`](./struct.ParkListCoaster.html) object.
#[derive(Debug, Clone)]
pub struct ParkListCoasterBuilder {
	body: self::ParkListCoaster,
}

impl ParkListCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
