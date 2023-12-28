/// Status
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatusListCoaster {
	pub name: Option<String>,
}

impl StatusListCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> StatusListCoasterBuilder {
		StatusListCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<StatusListCoaster> for StatusListCoasterBuilder {
	fn into(self) -> StatusListCoaster {
		self.body
	}
}

/// Builder for [`StatusListCoaster`](./struct.StatusListCoaster.html) object.
#[derive(Debug, Clone)]
pub struct StatusListCoasterBuilder {
	body: self::StatusListCoaster,
}

impl StatusListCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
