/// Coaster
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CoasterListCoaster {
	pub id: Option<i64>,
	pub name: String,
	pub park: Option<crate::park_list_coaster::ParkListCoaster>,
	pub rank: Option<i64>,
	pub status: Option<crate::status_list_coaster::StatusListCoaster>,
}

impl CoasterListCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> CoasterListCoasterBuilder<crate::generics::MissingName> {
		CoasterListCoasterBuilder {
			body: Default::default(),
			_name: core::marker::PhantomData,
		}
	}

	#[inline]
	pub fn get_coaster_collection() -> CoasterListCoasterGetBuilder {
		CoasterListCoasterGetBuilder {
			param_id: None,
			param_ids: None,
			param_name: None,
			param_manufacturer: None,
			param_manufacturers: None,
			param_order_id: None,
			param_order_rank: None,
			param_rank_between: None,
			param_rank_gt: None,
			param_rank_gte: None,
			param_rank_lt: None,
			param_rank_lte: None,
			param_total_ratings_between: None,
			param_total_ratings_gt: None,
			param_total_ratings_gte: None,
			param_total_ratings_lt: None,
			param_total_ratings_lte: None,
			param_exists_main_image: None,
			param_page: None,
		}
	}
}

impl Into<CoasterListCoaster> for CoasterListCoasterBuilder<crate::generics::NameExists> {
	fn into(self) -> CoasterListCoaster {
		self.body
	}
}

/// Builder for [`CoasterListCoaster`](./struct.CoasterListCoaster.html) object.
#[derive(Debug, Clone)]
pub struct CoasterListCoasterBuilder<Name> {
	body: self::CoasterListCoaster,
	_name: core::marker::PhantomData<Name>,
}

impl<Name> CoasterListCoasterBuilder<Name> {
	#[inline]
	pub fn id(mut self, value: impl Into<i64>) -> Self {
		self.body.id = Some(value.into());
		self
	}

	#[inline]
	pub fn name(
		mut self,
		value: impl Into<String>,
	) -> CoasterListCoasterBuilder<crate::generics::NameExists> {
		self.body.name = value.into();
		unsafe { std::mem::transmute(self) }
	}

	#[inline]
	pub fn park(mut self, value: crate::park_list_coaster::ParkListCoaster) -> Self {
		self.body.park = Some(value.into());
		self
	}

	#[inline]
	pub fn rank(mut self, value: impl Into<i64>) -> Self {
		self.body.rank = Some(value.into());
		self
	}

	#[inline]
	pub fn status(mut self, value: crate::status_list_coaster::StatusListCoaster) -> Self {
		self.body.status = Some(value.into());
		self
	}
}

/// Builder created by [`CoasterListCoaster::get_coaster_collection`](./struct.CoasterListCoaster.html#method.get_coaster_collection) method for a `GET` operation associated with `CoasterListCoaster`.
#[derive(Debug, Clone)]
pub struct CoasterListCoasterGetBuilder {
	param_id: Option<i64>,
	param_ids: Option<crate::util::Delimited<i64, crate::util::Multi>>,
	param_name: Option<String>,
	param_manufacturer: Option<String>,
	param_manufacturers: Option<crate::util::Delimited<String, crate::util::Multi>>,
	param_order_id: Option<String>,
	param_order_rank: Option<String>,
	param_rank_between: Option<String>,
	param_rank_gt: Option<String>,
	param_rank_gte: Option<String>,
	param_rank_lt: Option<String>,
	param_rank_lte: Option<String>,
	param_total_ratings_between: Option<String>,
	param_total_ratings_gt: Option<String>,
	param_total_ratings_gte: Option<String>,
	param_total_ratings_lt: Option<String>,
	param_total_ratings_lte: Option<String>,
	param_exists_main_image: Option<bool>,
	param_page: Option<i64>,
}

impl CoasterListCoasterGetBuilder {
	#[inline]
	pub fn id(mut self, value: impl Into<i64>) -> Self {
		self.param_id = Some(value.into());
		self
	}

	#[inline]
	pub fn ids(mut self, value: impl Iterator<Item = impl Into<i64>>) -> Self {
		self.param_ids = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
		self
	}

	#[inline]
	pub fn name(mut self, value: impl Into<String>) -> Self {
		self.param_name = Some(value.into());
		self
	}

	#[inline]
	pub fn manufacturer(mut self, value: impl Into<String>) -> Self {
		self.param_manufacturer = Some(value.into());
		self
	}

	#[inline]
	pub fn manufacturers(mut self, value: impl Iterator<Item = impl Into<String>>) -> Self {
		self.param_manufacturers = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
		self
	}

	#[inline]
	pub fn order_id(mut self, value: impl Into<String>) -> Self {
		self.param_order_id = Some(value.into());
		self
	}

	#[inline]
	pub fn order_rank(mut self, value: impl Into<String>) -> Self {
		self.param_order_rank = Some(value.into());
		self
	}

	#[inline]
	pub fn rank_between(mut self, value: impl Into<String>) -> Self {
		self.param_rank_between = Some(value.into());
		self
	}

	#[inline]
	pub fn rank_gt(mut self, value: impl Into<String>) -> Self {
		self.param_rank_gt = Some(value.into());
		self
	}

	#[inline]
	pub fn rank_gte(mut self, value: impl Into<String>) -> Self {
		self.param_rank_gte = Some(value.into());
		self
	}

	#[inline]
	pub fn rank_lt(mut self, value: impl Into<String>) -> Self {
		self.param_rank_lt = Some(value.into());
		self
	}

	#[inline]
	pub fn rank_lte(mut self, value: impl Into<String>) -> Self {
		self.param_rank_lte = Some(value.into());
		self
	}

	#[inline]
	pub fn total_ratings_between(mut self, value: impl Into<String>) -> Self {
		self.param_total_ratings_between = Some(value.into());
		self
	}

	#[inline]
	pub fn total_ratings_gt(mut self, value: impl Into<String>) -> Self {
		self.param_total_ratings_gt = Some(value.into());
		self
	}

	#[inline]
	pub fn total_ratings_gte(mut self, value: impl Into<String>) -> Self {
		self.param_total_ratings_gte = Some(value.into());
		self
	}

	#[inline]
	pub fn total_ratings_lt(mut self, value: impl Into<String>) -> Self {
		self.param_total_ratings_lt = Some(value.into());
		self
	}

	#[inline]
	pub fn total_ratings_lte(mut self, value: impl Into<String>) -> Self {
		self.param_total_ratings_lte = Some(value.into());
		self
	}

	#[inline]
	pub fn exists_main_image(mut self, value: impl Into<bool>) -> Self {
		self.param_exists_main_image = Some(value.into());
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
	for CoasterListCoasterGetBuilder
{
	type Output = Vec<CoasterListCoaster>;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		"/api/coasters".into()
	}

	fn modify(
		&self,
		req: Client::Request,
	) -> Result<Client::Request, crate::client::ApiError<Client::Response>> {
		use crate::client::Request;
		Ok(req
			.query(&[
				(
					"id",
					self.param_id.as_ref().map(std::string::ToString::to_string),
				),
				(
					"name",
					self.param_name
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"manufacturer",
					self.param_manufacturer
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"order[id]",
					self.param_order_id
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"order[rank]",
					self.param_order_rank
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"rank[between]",
					self.param_rank_between
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"rank[gt]",
					self.param_rank_gt
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"rank[gte]",
					self.param_rank_gte
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"rank[lt]",
					self.param_rank_lt
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"rank[lte]",
					self.param_rank_lte
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"totalRatings[between]",
					self.param_total_ratings_between
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"totalRatings[gt]",
					self.param_total_ratings_gt
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"totalRatings[gte]",
					self.param_total_ratings_gte
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"totalRatings[lt]",
					self.param_total_ratings_lt
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"totalRatings[lte]",
					self.param_total_ratings_lte
						.as_ref()
						.map(std::string::ToString::to_string),
				),
				(
					"exists[mainImage]",
					self.param_exists_main_image
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
					.param_ids
					.as_ref()
					.map(|v| {
						v.iter()
							.map(|v| ("id[]", v.to_string()))
							.collect::<Vec<_>>()
					})
					.unwrap_or_default()
			})
			.query({
				&self
					.param_manufacturers
					.as_ref()
					.map(|v| {
						v.iter()
							.map(|v| ("manufacturer[]", v.to_string()))
							.collect::<Vec<_>>()
					})
					.unwrap_or_default()
			}))
	}
}
