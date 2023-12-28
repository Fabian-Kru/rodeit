/// Coaster
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CoasterReadCoaster {
	#[serde(rename = "closingDate")]
	pub closing_date: Option<String>,
	pub height: Option<i64>,
	pub id: Option<i64>,
	#[serde(rename = "inversionsNumber")]
	pub inversions_number: Option<i64>,
	pub launchs: Option<Vec<crate::launch_read_coaster::LaunchReadCoaster>>,
	pub length: Option<i64>,
	#[serde(rename = "mainImage")]
	pub main_image: Option<crate::image_read_coaster::ImageReadCoaster>,
	pub manufacturer: Option<crate::manufacturer_read_coaster::ManufacturerReadCoaster>,
	#[serde(rename = "materialType")]
	pub material_type: Option<crate::material_type_read_coaster::MaterialTypeReadCoaster>,
	pub model: Option<crate::model_read_coaster::ModelReadCoaster>,
	pub name: String,
	#[serde(rename = "openingDate")]
	pub opening_date: Option<String>,
	pub park: Option<crate::park_read_coaster::ParkReadCoaster>,
	pub rank: Option<i64>,
	pub restraint: Option<crate::restraint_read_coaster::RestraintReadCoaster>,
	pub score: Option<String>,
	#[serde(rename = "seatingType")]
	pub seating_type: Option<crate::seating_type_read_coaster::SeatingTypeReadCoaster>,
	pub speed: Option<i64>,
	pub status: Option<crate::status_read_coaster::StatusReadCoaster>,
	#[serde(rename = "totalRatings")]
	pub total_ratings: Option<i64>,
	#[serde(rename = "validDuels")]
	pub valid_duels: Option<i64>,
}

impl CoasterReadCoaster {
	/// Create a builder for this object.
	#[inline]
	pub fn builder() -> CoasterReadCoasterBuilder<crate::generics::MissingName> {
		CoasterReadCoasterBuilder {
			body: Default::default(),
			_name: core::marker::PhantomData,
		}
	}

	#[inline]
	pub fn get_coaster_item() -> CoasterReadCoasterGetBuilder<crate::generics::MissingId> {
		CoasterReadCoasterGetBuilder {
			inner: Default::default(),
			_param_id: core::marker::PhantomData,
		}
	}
}

impl Into<CoasterReadCoaster> for CoasterReadCoasterBuilder<crate::generics::NameExists> {
	fn into(self) -> CoasterReadCoaster {
		self.body
	}
}

/// Builder for [`CoasterReadCoaster`](./struct.CoasterReadCoaster.html) object.
#[derive(Debug, Clone)]
pub struct CoasterReadCoasterBuilder<Name> {
	body: self::CoasterReadCoaster,
	_name: core::marker::PhantomData<Name>,
}

impl<Name> CoasterReadCoasterBuilder<Name> {
	#[inline]
	pub fn closing_date(mut self, value: impl Into<String>) -> Self {
		self.body.closing_date = Some(value.into());
		self
	}

	#[inline]
	pub fn height(mut self, value: impl Into<i64>) -> Self {
		self.body.height = Some(value.into());
		self
	}

	#[inline]
	pub fn id(mut self, value: impl Into<i64>) -> Self {
		self.body.id = Some(value.into());
		self
	}

	#[inline]
	pub fn inversions_number(mut self, value: impl Into<i64>) -> Self {
		self.body.inversions_number = Some(value.into());
		self
	}

	#[inline]
	pub fn launchs(
		mut self,
		value: impl Iterator<Item = crate::launch_read_coaster::LaunchReadCoaster>,
	) -> Self {
		self.body.launchs = Some(value.map(|value| value.into()).collect::<Vec<_>>().into());
		self
	}

	#[inline]
	pub fn length(mut self, value: impl Into<i64>) -> Self {
		self.body.length = Some(value.into());
		self
	}

	#[inline]
	pub fn main_image(mut self, value: crate::image_read_coaster::ImageReadCoaster) -> Self {
		self.body.main_image = Some(value.into());
		self
	}

	#[inline]
	pub fn manufacturer(
		mut self,
		value: crate::manufacturer_read_coaster::ManufacturerReadCoaster,
	) -> Self {
		self.body.manufacturer = Some(value.into());
		self
	}

	#[inline]
	pub fn material_type(
		mut self,
		value: crate::material_type_read_coaster::MaterialTypeReadCoaster,
	) -> Self {
		self.body.material_type = Some(value.into());
		self
	}

	#[inline]
	pub fn model(mut self, value: crate::model_read_coaster::ModelReadCoaster) -> Self {
		self.body.model = Some(value.into());
		self
	}

	#[inline]
	pub fn name(
		mut self,
		value: impl Into<String>,
	) -> CoasterReadCoasterBuilder<crate::generics::NameExists> {
		self.body.name = value.into();
		unsafe { std::mem::transmute(self) }
	}

	#[inline]
	pub fn opening_date(mut self, value: impl Into<String>) -> Self {
		self.body.opening_date = Some(value.into());
		self
	}

	#[inline]
	pub fn park(mut self, value: crate::park_read_coaster::ParkReadCoaster) -> Self {
		self.body.park = Some(value.into());
		self
	}

	#[inline]
	pub fn rank(mut self, value: impl Into<i64>) -> Self {
		self.body.rank = Some(value.into());
		self
	}

	#[inline]
	pub fn restraint(mut self, value: crate::restraint_read_coaster::RestraintReadCoaster) -> Self {
		self.body.restraint = Some(value.into());
		self
	}

	#[inline]
	pub fn score(mut self, value: impl Into<String>) -> Self {
		self.body.score = Some(value.into());
		self
	}

	#[inline]
	pub fn seating_type(
		mut self,
		value: crate::seating_type_read_coaster::SeatingTypeReadCoaster,
	) -> Self {
		self.body.seating_type = Some(value.into());
		self
	}

	#[inline]
	pub fn speed(mut self, value: impl Into<i64>) -> Self {
		self.body.speed = Some(value.into());
		self
	}

	#[inline]
	pub fn status(mut self, value: crate::status_read_coaster::StatusReadCoaster) -> Self {
		self.body.status = Some(value.into());
		self
	}

	#[inline]
	pub fn total_ratings(mut self, value: impl Into<i64>) -> Self {
		self.body.total_ratings = Some(value.into());
		self
	}

	#[inline]
	pub fn valid_duels(mut self, value: impl Into<i64>) -> Self {
		self.body.valid_duels = Some(value.into());
		self
	}
}

/// Builder created by [`CoasterReadCoaster::get_coaster_item`](./struct.CoasterReadCoaster.html#method.get_coaster_item) method for a `GET` operation associated with `CoasterReadCoaster`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct CoasterReadCoasterGetBuilder<Id> {
	inner: CoasterReadCoasterGetBuilderContainer,
	_param_id: core::marker::PhantomData<Id>,
}

#[derive(Debug, Default, Clone)]
struct CoasterReadCoasterGetBuilderContainer {
	param_id: Option<String>,
}

impl<Id> CoasterReadCoasterGetBuilder<Id> {
	#[inline]
	pub fn id(
		mut self,
		value: impl Into<String>,
	) -> CoasterReadCoasterGetBuilder<crate::generics::IdExists> {
		self.inner.param_id = Some(value.into());
		unsafe { std::mem::transmute(self) }
	}
}

impl<Client: crate::client::ApiClient + Sync + 'static> crate::client::Sendable<Client>
	for CoasterReadCoasterGetBuilder<crate::generics::IdExists>
{
	type Output = CoasterReadCoaster;

	const METHOD: http::Method = http::Method::GET;

	fn rel_path(&self) -> std::borrow::Cow<'static, str> {
		format!(
			"/api/coasters/{id}",
			id = self.inner.param_id.as_ref().expect("missing parameter id?")
		)
		.into()
	}
}
