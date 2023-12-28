///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SeatingTypeReadCoaster {
	pub name: Option<String>,
}

impl SeatingTypeReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> SeatingTypeReadCoasterBuilder {
		SeatingTypeReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<SeatingTypeReadCoaster> for SeatingTypeReadCoasterBuilder {
	fn into(self) -> SeatingTypeReadCoaster {
		self.body
	}
}

/// Builder for [`SeatingTypeReadCoaster`](./struct.SeatingTypeReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct SeatingTypeReadCoasterBuilder {
	body: self::SeatingTypeReadCoaster,
}

impl SeatingTypeReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
