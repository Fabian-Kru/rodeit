///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MaterialTypeReadCoaster {
	pub name: Option<String>,
}

impl MaterialTypeReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> MaterialTypeReadCoasterBuilder {
		MaterialTypeReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<MaterialTypeReadCoaster> for MaterialTypeReadCoasterBuilder {
	fn into(self) -> MaterialTypeReadCoaster {
		self.body
	}
}

/// Builder for [`MaterialTypeReadCoaster`](./struct.MaterialTypeReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct MaterialTypeReadCoasterBuilder {
	body: self::MaterialTypeReadCoaster,
}

impl MaterialTypeReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
