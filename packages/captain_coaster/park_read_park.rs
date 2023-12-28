/// Park
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ParkReadPark {
	pub country: Option<crate::country_read_park::CountryReadPark>,
	pub id: Option<i64>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
	pub name: Option<String>,
}

impl ParkReadPark {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ParkReadParkBuilder {
		ParkReadParkBuilder {
			body: Default::default(),
		}
	}

	#[inline]
	pub fn get_park_collection() -> ParkReadParkGetBuilder {
		ParkReadParkGetBuilder { param_page: None }
	}

	#[inline]
	pub fn get_park_item() -> ParkReadParkGetBuilder1<crate::generics::MissingId> {
		ParkReadParkGetBuilder1 {
			inner: Default::default(),
			_param_id: core::marker::PhantomData,
		}
	}
}

impl Into<ParkReadPark> for ParkReadParkBuilder {
	fn into(self) -> ParkReadPark {
		self.body
	}
}

/// Builder for [`ParkReadPark`](./struct.ParkReadPark.html) object.
#[derive(Debug, Clone)]
pub struct ParkReadParkBuilder {
	body: self::ParkReadPark,
}

impl ParkReadParkBuilder {
	#[inline]
	pub fn country(mut self, value: crate::country_read_park::CountryReadPark) -> Self {
		self.body.country = Some(value.into());
		self
	}

	#[inline]
	pub fn id(mut self, value: impl Into<i64>) -> Self {
		self.body.id = Some(value.into());
		self
	}

	#[inline]
	pub fn latitude(mut self, value: impl Into<f64>) -> Self {
		self.body.latitude = Some(value.into());
		self
	}

	#[inline]
	pub fn longitude(mut self, value: impl Into<f64>) -> Self {
		self.body.longitude = Some(value.into());
		self
	}

	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}

/// Builder created by [`ParkReadPark::get_park_collection`](./struct.ParkReadPark.html#method.get_park_collection) method for a `GET` operation associated with `ParkReadPark`.
#[derive(Debug, Clone)]
pub struct ParkReadParkGetBuilder {
	param_page: Option<i64>,
}

impl ParkReadParkGetBuilder {
	/// The collection page number
	#[inline]
	pub fn page(mut self, value: impl Into<i64>) -> Self {
		self.param_page = Some(value.into());
		self
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for ParkReadParkGetBuilder
{
	type Output = Vec<ParkReadPark>;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		"/api/parks".into()
	}

	fn modify(
		&self,
		req: Client::Request,
	) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
		use crate::client::Request;
		Ok(req.query(&[(
			"page",
			self.param_page
				.as_ref()
				.map(std::string::ToString::to_string),
		)]))
	}
}

/// Builder created by [`ParkReadPark::get_park_item`](./struct.ParkReadPark.html#method.get_park_item) method for a `GET` operation associated with `ParkReadPark`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ParkReadParkGetBuilder1<Id> {
	inner: ParkReadParkGetBuilder1Container,
	_param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct ParkReadParkGetBuilder1Container {
	param_id: Option<String>,
}

impl<Id> ParkReadParkGetBuilder1<Id> {
	#[inline]
	pub fn id(
		mut self,
		value: impl Into<String>,
	) -> ParkReadParkGetBuilder1<crate::generics::IdExists> {
		self.inner.param_id = Some(value.into());
		unsafe { std::mem::transmute(self) }
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for ParkReadParkGetBuilder1<crate::generics::IdExists>
{
	type Output = ParkReadPark;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		format!(
			"/api/parks/{id}",
			id = self.inner.param_id.as_ref().expect("missing parameter id?")
		)
		.into()
	}
}
