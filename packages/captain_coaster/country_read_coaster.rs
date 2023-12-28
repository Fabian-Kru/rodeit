///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CountryReadCoaster {
	pub name: Option<String>,
}

impl CountryReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> CountryReadCoasterBuilder {
		CountryReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<CountryReadCoaster> for CountryReadCoasterBuilder {
	fn into(self) -> CountryReadCoaster {
		self.body
	}
}

/// Builder for [`CountryReadCoaster`](./struct.CountryReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct CountryReadCoasterBuilder {
	body: self::CountryReadCoaster,
}

impl CountryReadCoasterBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
