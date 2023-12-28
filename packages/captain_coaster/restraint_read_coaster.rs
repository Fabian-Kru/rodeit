/// Restraint
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RestraintReadCoaster {
	pub name: Option<String>,
}

impl RestraintReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> RestraintReadCoasterBuilder {
		RestraintReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<RestraintReadCoaster> for RestraintReadCoasterBuilder {
	fn into(self) -> RestraintReadCoaster {
		self.body
	}
}

/// Builder for [`RestraintReadCoaster`](./struct.RestraintReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct RestraintReadCoasterBuilder {
	body: self::RestraintReadCoaster,
}

impl RestraintReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
