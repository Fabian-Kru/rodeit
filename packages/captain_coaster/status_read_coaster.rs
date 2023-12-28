/// Status
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatusReadCoaster {
	pub name: Option<String>,
}

impl StatusReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> StatusReadCoasterBuilder {
		StatusReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<StatusReadCoaster> for StatusReadCoasterBuilder {
	fn into(self) -> StatusReadCoaster {
		self.body
	}
}

/// Builder for [`StatusReadCoaster`](./struct.StatusReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct StatusReadCoasterBuilder {
	body: self::StatusReadCoaster,
}

impl StatusReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
