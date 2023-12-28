/// Badge
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImageReadCoaster {
	pub path: Option<String>,
}

impl ImageReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ImageReadCoasterBuilder {
		ImageReadCoasterBuilder {
			body: Default::default(),
		}
	}
}

impl Into<ImageReadCoaster> for ImageReadCoasterBuilder {
	fn into(self) -> ImageReadCoaster {
		self.body
	}
}

/// Builder for [`ImageReadCoaster`](./struct.ImageReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct ImageReadCoasterBuilder {
	body: self::ImageReadCoaster,
}

impl ImageReadCoasterBuilder {
	#[inline]
	pub fn path(mut self, value: impl Into<String>) -> Self {
		self.body.path = Some(value.into());
		self
	}
}
