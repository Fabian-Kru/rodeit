/// Launch
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LaunchReadCoaster {
	pub name: Option<String>,
}

impl LaunchReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> LaunchReadCoasterBuilder {
		LaunchReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<LaunchReadCoaster> for LaunchReadCoasterBuilder {
	fn into(self) -> LaunchReadCoaster {
		self.body
	}
}

/// Builder for [`LaunchReadCoaster`](./struct.LaunchReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct LaunchReadCoasterBuilder {
	body: self::LaunchReadCoaster,
}

impl LaunchReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
