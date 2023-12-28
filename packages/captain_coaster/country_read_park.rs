///
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CountryReadPark {
	pub name: Option<String>,
}

impl CountryReadPark {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> CountryReadParkBuilder {
		CountryReadParkBuilder {
			body: Default::default(),
		}
	}
}

impl Into<CountryReadPark> for CountryReadParkBuilder {
	fn into(self) -> CountryReadPark {
		self.body
	}
}

/// Builder for [`CountryReadPark`](./struct.CountryReadPark.html) object.
#[derive(Debug, Clone)]
pub struct CountryReadParkBuilder {
	body: self::CountryReadPark,
}

impl CountryReadParkBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}
