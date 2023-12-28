/// Status
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ModelReadModel {
	pub name: Option<String>,
}

impl ModelReadModel {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> ModelReadModelBuilder {
		ModelReadModelBuilder {
			body: Default::default(),
		}
	}

	#[inline]
	pub fn get_model_collection() -> ModelReadModelGetBuilder {
		ModelReadModelGetBuilder { param_page: None }
	}

	#[inline]
	pub fn get_model_item() -> ModelReadModelGetBuilder1<crate::generics::MissingId> {
		ModelReadModelGetBuilder1 {
			inner: Default::default(),
			_param_id: core::marker::PhantomData,
		}
	}
}

impl Into<ModelReadModel> for ModelReadModelBuilder {
	fn into(self) -> ModelReadModel {
		self.body
	}
}

/// Builder for [`ModelReadModel`](./struct.ModelReadModel.html) object.
#[derive(Debug, Clone)]
pub struct ModelReadModelBuilder {
	body: self::ModelReadModel,
}

impl ModelReadModelBuilder {
	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.body.name = Some(value.into());
		self
	}
}

/// Builder created by [`ModelReadModel::get_model_collection`](./struct.ModelReadModel.html#method.get_model_collection) method for a `GET` operation associated with `ModelReadModel`.
#[derive(Debug, Clone)]
pub struct ModelReadModelGetBuilder {
	param_page: Option<i64>,
}

impl ModelReadModelGetBuilder {
	/// The collection page number
	#[inline]
	pub fn page(mut self, value: impl Into<i64>) -> Self {
		self.param_page = Some(value.into());
		self
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for ModelReadModelGetBuilder
{
	type Output = Vec<ModelReadModel>;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		"/api/models".into()
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

/// Builder created by [`ModelReadModel::get_model_item`](./struct.ModelReadModel.html#method.get_model_item) method for a `GET` operation associated with `ModelReadModel`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct ModelReadModelGetBuilder1<Id> {
	inner: ModelReadModelGetBuilder1Container,
	_param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct ModelReadModelGetBuilder1Container {
	param_id: Option<String>,
}

impl<Id> ModelReadModelGetBuilder1<Id> {
	#[inline]
	pub fn id(
		mut self,
		value: impl Into<String>,
	) -> ModelReadModelGetBuilder1<crate::generics::IdExists> {
		self.inner.param_id = Some(value.into());
		unsafe { std::mem::transmute(self) }
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for ModelReadModelGetBuilder1<crate::generics::IdExists>
{
	type Output = ModelReadModel;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		format!(
			"/api/models/{id}",
			id = self.inner.param_id.as_ref().expect("missing parameter id?")
		)
		.into()
	}
}
