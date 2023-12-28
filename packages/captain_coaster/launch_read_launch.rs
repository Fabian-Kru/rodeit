/// Launch
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LaunchReadLaunch {
	pub name: Option<String>,
}

impl LaunchReadLaunch {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> LaunchReadLaunchBuilder {
		LaunchReadLaunchBuilder {
			body: Default::default(),
		}
	}

	#[inline]
	pub fn get_launch_collection() -> LaunchReadLaunchGetBuilder {
		LaunchReadLaunchGetBuilder { param_page: None }
	}

	#[inline]
	pub fn get_launch_item() -> LaunchReadLaunchGetBuilder1<crate::generics::MissingId> {
		LaunchReadLaunchGetBuilder1 {
			inner: Default::default(),
			_param_id: core::marker::PhantomData,
		}
	}
}

impl Into<LaunchReadLaunch> for LaunchReadLaunchBuilder {
	fn into(self) -> LaunchReadLaunch {
		self.body
	}
}

/// Builder for [`LaunchReadLaunch`](./struct.LaunchReadLaunch.html) object.
#[derive(Debug, Clone)]
pub struct LaunchReadLaunchBuilder {
	body: self::LaunchReadLaunch,
}

impl LaunchReadLaunchBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}

/// Builder created by [`LaunchReadLaunch::get_launch_collection`](./struct.LaunchReadLaunch.html#method.get_launch_collection) method for a `GET` operation associated with `LaunchReadLaunch`.
#[derive(Debug, Clone)]
pub struct LaunchReadLaunchGetBuilder {
	param_page: Option<i64>,
}

impl LaunchReadLaunchGetBuilder {
	/// The collection page number
	#[inline]
	pub fn page(mut self, value: impl Into<i64>) -> Self {
		self.param_page = Some(value.into());
		self
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for LaunchReadLaunchGetBuilder
{
	type Output = Vec<LaunchReadLaunch>;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		"/api/launches".into()
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

/// Builder created by [`LaunchReadLaunch::get_launch_item`](./struct.LaunchReadLaunch.html#method.get_launch_item) method for a `GET` operation associated with `LaunchReadLaunch`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct LaunchReadLaunchGetBuilder1<Id> {
	inner: LaunchReadLaunchGetBuilder1Container,
	_param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct LaunchReadLaunchGetBuilder1Container {
	param_id: Option<String>,
}

impl<Id> LaunchReadLaunchGetBuilder1<Id> {
	#[inline]
	pub fn id(
		mut self,
		value: impl Into<String>,
	) -> LaunchReadLaunchGetBuilder1<crate::generics::IdExists> {
		self.inner.param_id = Some(value.into());
		unsafe { std::mem::transmute(self) }
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for LaunchReadLaunchGetBuilder1<crate::generics::IdExists>
{
	type Output = LaunchReadLaunch;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		format!(
			"/api/launches/{id}",
			id = self.inner.param_id.as_ref().expect("missing parameter id?")
		)
		.into()
	}
}
