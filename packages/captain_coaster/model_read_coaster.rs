/// Status
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ModelReadCoaster {
	pub name: Option<String>,
}

impl ModelReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ModelReadCoasterBuilder {
		ModelReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<ModelReadCoaster> for ModelReadCoasterBuilder {
	fn into(self) -> ModelReadCoaster {
		self.body
	}
}

/// Builder for [`ModelReadCoaster`](./struct.ModelReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct ModelReadCoasterBuilder {
	body: self::ModelReadCoaster,
}

impl ModelReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
