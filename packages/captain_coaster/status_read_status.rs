/// Status
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StatusReadStatus {
	pub id: Option<i64>,
	#[serde(rename = "isRateable")]
	pub is_rateable: Option<bool>,
	pub name: Option<String>,
}

impl StatusReadStatus {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> StatusReadStatusBuilder {
		StatusReadStatusBuilder {
			body: Default::default(),
		}
	}

	#[inline]
	pub fn get_status_collection() -> StatusReadStatusGetBuilder {
		StatusReadStatusGetBuilder { param_page: None }
	}

	#[inline]
	pub fn get_status_item() -> StatusReadStatusGetBuilder1<crate::generics::MissingId> {
		StatusReadStatusGetBuilder1 {
			inner: Default::default(),
			_param_id: core::marker::PhantomData,
		}
	}
}

impl Into<StatusReadStatus> for StatusReadStatusBuilder {
	fn into(self) -> StatusReadStatus {
		self.body
	}
}

/// Builder for [`StatusReadStatus`](./struct.StatusReadStatus.html) object.
#[derive(Debug, Clone)]
pub struct StatusReadStatusBuilder {
	body: self::StatusReadStatus,
}

impl StatusReadStatusBuilder {
	#[inline]
	pub fn id(mut self, value: impl Into<i64>) -> Self {
		self.body.id = Some(value.into());
		self
	}

	#[inline]
	pub fn is_rateable(mut self, value: impl Into<bool>) -> Self {
		self.body.is_rateable = Some(value.into());
		self
	}

	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}

/// Builder created by [`StatusReadStatus::get_status_collection`](./struct.StatusReadStatus.html#method.get_status_collection) method for a `GET` operation associated with `StatusReadStatus`.
#[derive(Debug, Clone)]
pub struct StatusReadStatusGetBuilder {
	param_page: Option<i64>,
}

impl StatusReadStatusGetBuilder {
	/// The collection page number
	#[inline]
	pub fn page(mut self, value: impl Into<i64>) -> Self {
		self.param_page = Some(value.into());
		self
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for StatusReadStatusGetBuilder
{
	type Output = Vec<StatusReadStatus>;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		"/api/statuses".into()
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

/// Builder created by [`StatusReadStatus::get_status_item`](./struct.StatusReadStatus.html#method.get_status_item) method for a `GET` operation associated with `StatusReadStatus`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct StatusReadStatusGetBuilder1<Id> {
	inner: StatusReadStatusGetBuilder1Container,
	_param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct StatusReadStatusGetBuilder1Container {
	param_id: Option<String>,
}

impl<Id> StatusReadStatusGetBuilder1<Id> {
	#[inline]
	pub fn id(
		mut self,
		value: impl Into<String>,
	) -> StatusReadStatusGetBuilder1<crate::generics::IdExists> {
		self.inner.param_id = Some(value.into());
		unsafe { std::mem::transmute(self) }
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for StatusReadStatusGetBuilder1<crate::generics::IdExists>
{
	type Output = StatusReadStatus;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		format!(
			"/api/statuses/{id}",
			id = self.inner.param_id.as_ref().expect("missing parameter id?")
		)
		.into()
	}
}
