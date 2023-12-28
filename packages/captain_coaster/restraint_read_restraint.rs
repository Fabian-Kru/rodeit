/// Restraint
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RestraintReadRestraint {
	pub name: Option<String>,
}

impl RestraintReadRestraint {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> RestraintReadRestraintBuilder {
		RestraintReadRestraintBuilder {
			body: Default::default(),
		}
	}

	#[inline]
	pub fn get_restraint_collection() -> RestraintReadRestraintGetBuilder {
		RestraintReadRestraintGetBuilder { param_page: None }
	}

	#[inline]
	pub fn get_restraint_item() -> RestraintReadRestraintGetBuilder1<crate::generics::MissingId> {
		RestraintReadRestraintGetBuilder1 {
			inner: Default::default(),
			_param_id: core::marker::PhantomData,
		}
	}
}

impl Into<RestraintReadRestraint> for RestraintReadRestraintBuilder {
	fn into(self) -> RestraintReadRestraint {
		self.body
	}
}

/// Builder for [`RestraintReadRestraint`](./struct.RestraintReadRestraint.html) object.
#[derive(Debug, Clone)]
pub struct RestraintReadRestraintBuilder {
	body: self::RestraintReadRestraint,
}

impl RestraintReadRestraintBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}

/// Builder created by [`RestraintReadRestraint::get_restraint_collection`](./struct.RestraintReadRestraint.html#method.get_restraint_collection) method for a `GET` operation associated with `RestraintReadRestraint`.
#[derive(Debug, Clone)]
pub struct RestraintReadRestraintGetBuilder {
	param_page: Option<i64>,
}

impl RestraintReadRestraintGetBuilder {
	/// The collection page number
	#[inline]
	pub fn page(mut self, value: impl Into<i64>) -> Self {
		self.param_page = Some(value.into());
		self
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for RestraintReadRestraintGetBuilder
{
	type Output = Vec<RestraintReadRestraint>;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		"/api/restraints".into()
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

/// Builder created by [`RestraintReadRestraint::get_restraint_item`](./struct.RestraintReadRestraint.html#method.get_restraint_item) method for a `GET` operation associated with `RestraintReadRestraint`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct RestraintReadRestraintGetBuilder1<Id> {
	inner: RestraintReadRestraintGetBuilder1Container,
	_param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct RestraintReadRestraintGetBuilder1Container {
	param_id: Option<String>,
}

impl<Id> RestraintReadRestraintGetBuilder1<Id> {
	#[inline]
	pub fn id(
		mut self,
		value: impl Into<String>,
	) -> RestraintReadRestraintGetBuilder1<crate::generics::IdExists> {
		self.inner.param_id = Some(value.into());
		unsafe { std::mem::transmute(self) }
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for RestraintReadRestraintGetBuilder1<crate::generics::IdExists>
{
	type Output = RestraintReadRestraint;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		format!(
			"/api/restraints/{id}",
			id = self.inner.param_id.as_ref().expect("missing parameter id?")
		)
		.into()
	}
}
