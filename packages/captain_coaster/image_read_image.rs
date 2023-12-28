/// Badge
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ImageReadImage {
	pub coaster: Option<String>,
	pub credit: String,
	pub path: Option<String>,
}

impl ImageReadImage {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ImageReadImageBuilder<crate::generics::MissingCredit> {
		ImageReadImageBuilder {
			body: Default::default(),
			_credit: core::marker::PhantomData,
		}
	}

	#[inline]
	pub fn get_image_collection() -> ImageReadImageGetBuilder {
		ImageReadImageGetBuilder {
			param_coaster: None,
			param_coasters: None,
			param_page: None,
		}
	}

	#[inline]
	pub fn get_image_item() -> ImageReadImageGetBuilder1<crate::generics::MissingId> {
		ImageReadImageGetBuilder1 {
			inner: Default::default(),
			_param_id: core::marker::PhantomData,
		}
	}
}

impl Into<ImageReadImage> for ImageReadImageBuilder<crate::generics::CreditExists> {
	fn into(self) -> ImageReadImage {
		self.body
	}
}

/// Builder for [`ImageReadImage`](./struct.ImageReadImage.html) object.
#[derive(Debug, Clone)]
pub struct ImageReadImageBuilder<Credit> {
	body: self::ImageReadImage,
	_credit: core::marker::PhantomData<Credit>,
}

impl<Credit> ImageReadImageBuilder<Credit> {
	#[inline]
	pub fn coaster(mut self, value: impl Into<String>) -> Self {
		self.body.coaster = Some(value.into());
		self
	}

	#[inline]
	pub fn credit(
		mut self,
		value: impl Into<String>,
	) -> ImageReadImageBuilder<crate::generics::CreditExists> {
		self.body.credit = value.into();
		unsafe { std::mem::transmute(self) }
	}

	#[inline]
	pub fn path(mut self, value: impl Into<String>) -> Self {
		self.body.path = Some(value.into());
		self
	}
}

/// Builder created by [`ImageReadImage::get_image_collection`](./struct.ImageReadImage.html#method.get_image_collection) method for a `GET` operation associated with `ImageReadImage`.
#[derive(Debug, Clone)]
pub struct ImageReadImageGetBuilder {
	param_coaster: Option<String>,
	param_coasters: Option<crate::util::Delimited<String, crate::util::Multi>>,
	param_page: Option<i64>,
}

impl ImageReadImageGetBuilder {
	#[inline]
	pub fn coaster(mut self, value: impl Into<String>) -> Self {
		self.param_coaster = Some(value.into());
		self
	}

	#[inline]
	pub fn coasters(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
		self.param_coasters = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
		self
	}

	/// The collection page number
	#[inline]
	pub fn page(mut self, value: impl Into<i64>) -> Self {
		self.param_page = Some(value.into());
		self
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for ImageReadImageGetBuilder
{
	type Output = Vec<ImageReadImage>;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		"/api/images".into()
	}

	fn modify(
		&self,
		req: Client::Request,
	) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
		use crate::client::Request;
		Ok(req
			.query(&[
				(
					"coaster",
					self.param_coaster
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"page",
					self.param_page
						.as_ref()
						.map(std::string::ToString::to_string),
				),
			])
			.query({
				&self
					.param_coasters
					.as_ref()
					.map(|v| {
						v.iter()
							.map(|v| ("coaster[]", v.to_string()))
							.collect::<Vec<_>>()
					})
					.unwrap_or_default()
			}))
	}
}

/// Builder created by [`ImageReadImage::get_image_item`](./struct.ImageReadImage.html#method.get_image_item) method for a `GET` operation associated with `ImageReadImage`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ImageReadImageGetBuilder1<Id> {
	inner: ImageReadImageGetBuilder1Container,
	_param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct ImageReadImageGetBuilder1Container {
	param_id: Option<String>,
}

impl<Id> ImageReadImageGetBuilder1<Id> {
	#[inline]
	pub fn id(
		mut self,
		value: impl Into<String>,
	) -> ImageReadImageGetBuilder1<crate::generics::IdExists> {
		self.inner.param_id = Some(value.into());
		unsafe { std::mem::transmute(self) }
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for ImageReadImageGetBuilder1<crate::generics::IdExists>
{
	type Output = ImageReadImage;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		format!(
			"/api/images/{id}",
			id = self.inner.param_id.as_ref().expect("missing parameter id?")
		)
		.into()
	}
}
