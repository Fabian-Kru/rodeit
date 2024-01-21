use std::{str::FromStr, sync::Arc};

use aide::{
	axum::{
		routing::{get_with, post_with},
		ApiRouter,
	},
	transform::TransformOperation,
};
use anyhow::Result;
use axum::{
	extract::{Path, State},
	http::StatusCode,
	Json,
};
use captain_coaster::{client::Sendable, coaster_read_coaster::CoasterReadCoaster};
use futures::future::join_all;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::opt::PatchOp;

use crate::AppState;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct BucketList {
	user_id: u32,
	coaster_ids: Vec<u32>,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
struct Park {
	id: u32,
	name: String,
	country: Option<Country>,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
struct Manufacturer {
	name: String,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
struct Coaster {
	id: u32,
	name: String,
	speed: Option<u32>,
	height: Option<u32>,
	inversions: Option<u32>,
	manufacturer: Option<Manufacturer>,
	park: Option<Park>,
	image: Option<String>,
}

#[derive(Clone, Serialize, JsonSchema)]
enum Country {
	Argentina,
	Australia,
	Austria,
	Belgium,
	Brazil,
	Burma,
	Canada,
	China,
	Colombia,
	Cyprus,
	CzechRepublic,
	Denmark,
	Finland,
	France,
	Germany,
	Guatemala,
	Hungary,
	India,
	Indonesia,
	Iraq,
	Ireland,
	Israel,
	Italy,
	Japan,
	Lebanon,
	Malaysia,
	Mexico,
	Mongolia,
	Netherlands,
	NewZealand,
	Norway,
	Peru,
	Poland,
	Portugal,
	Qatar,
	Russia,
	Singapore,
	SouthAfrica,
	SouthKorea,
	Spain,
	Sweden,
	Switzerland,
	Taiwan,
	Thailand,
	Turkey,
	Ukraine,
	UnitedArabEmirates,
	UnitedKingdom,
	UnitedStates,
	Vietnam,
}

impl Country {
	fn from_id(id: &str) -> Option<Self> {
		return id.split_once(".")?.1.parse().ok();
	}
}

impl FromStr for Country {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
		match s {
			"argentina" => Ok(Self::Argentina),
			"australia" => Ok(Self::Australia),
			"austria" => Ok(Self::Austria),
			"belgium" => Ok(Self::Belgium),
			"brazil" => Ok(Self::Brazil),
			"burma" => Ok(Self::Burma),
			"canada" => Ok(Self::Canada),
			"china" => Ok(Self::China),
			"colombia" => Ok(Self::Colombia),
			"cyprus" => Ok(Self::Cyprus),
			"czech" => Ok(Self::CzechRepublic),
			"denmark" => Ok(Self::Denmark),
			"finland" => Ok(Self::Finland),
			"france" => Ok(Self::France),
			"germany" => Ok(Self::Germany),
			"guatemala" => Ok(Self::Guatemala),
			"hungary" => Ok(Self::Hungary),
			"india" => Ok(Self::India),
			"indonesia" => Ok(Self::Indonesia),
			"iraq" => Ok(Self::Iraq),
			"ireland" => Ok(Self::Ireland),
			"israel" => Ok(Self::Israel),
			"italy" => Ok(Self::Italy),
			"japan" => Ok(Self::Japan),
			"lebanon" => Ok(Self::Lebanon),
			"malaysia" => Ok(Self::Malaysia),
			"mexico" => Ok(Self::Mexico),
			"mongolia" => Ok(Self::Mongolia),
			"netherlands" => Ok(Self::Netherlands),
			"newzealand" => Ok(Self::NewZealand),
			"norway" => Ok(Self::Norway),
			"peru" => Ok(Self::Peru),
			"poland" => Ok(Self::Poland),
			"portugal" => Ok(Self::Portugal),
			"qatar" => Ok(Self::Qatar),
			"russia" => Ok(Self::Russia),
			"singapore" => Ok(Self::Singapore),
			"southafrica" => Ok(Self::SouthAfrica),
			"southkorea" => Ok(Self::SouthKorea),
			"spain" => Ok(Self::Spain),
			"sweden" => Ok(Self::Sweden),
			"switzerland" => Ok(Self::Switzerland),
			"taiwan" => Ok(Self::Taiwan),
			"thailand" => Ok(Self::Thailand),
			"turkey" => Ok(Self::Turkey),
			"ukraine" => Ok(Self::Ukraine),
			"uae" => Ok(Self::UnitedArabEmirates),
			"uk" => Ok(Self::UnitedKingdom),
			"usa" => Ok(Self::UnitedStates),
			"vietnam" => Ok(Self::Vietnam),
			_ => Err(anyhow::anyhow!("Invalid country")),
		}
	}
}

pub fn create_router(state: Arc<AppState>) -> ApiRouter {
	ApiRouter::new()
		.api_route(
			"/:user_id",
			get_with(find_coasters_by_user_id, docs_find_coasters_by_user_id)
				.post_with(add_coaster, docs_add_coaster)
				.put_with(set_coasters, docs_set_coasters),
		)
		.api_route(
			"/:user_id/:index",
			post_with(insert_coaster, docs_insert_coaster)
				.delete_with(remove_coaster, docs_remove_coaster),
		)
		.with_state(state)
}

async fn find_coasters_by_user_id(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
) -> Result<Json<Vec<Coaster>>, StatusCode> {
	let store = &state.store;

	#[derive(Deserialize)]
	struct SelectBucketList {
		coaster_ids: Vec<u32>,
	}

	let bucket_list: SelectBucketList = store
		.select(("bucket_list", user_id))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
		.ok_or(StatusCode::NOT_FOUND)?;

	let coasters = bucket_list.coaster_ids.iter().map(|coaster_id| async {
		let raw_coaster = CoasterReadCoaster::get_coaster_item()
			.id(coaster_id.to_string())
			.send(&state.cc_client)
			.await
			.unwrap();
		// .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

		return Ok::<Coaster, StatusCode>(Coaster {
			id: *coaster_id,
			name: raw_coaster.name.clone(),
			speed: raw_coaster.speed.and_then(|speed| Some(speed as u32)),
			height: raw_coaster.height.and_then(|height| Some(height as u32)),
			inversions: raw_coaster
				.inversions_number
				.and_then(|inversions| Some(inversions as u32)),
			manufacturer: raw_coaster
				.manufacturer
				.clone()
				.map(|manufacturer| Manufacturer {
					name: manufacturer.name.unwrap(),
				}),
			park: raw_coaster.park.clone().map(|park| Park {
				id: park.id,
				name: park.name.unwrap(),
				country: Country::from_id(park.country.unwrap().name.unwrap().as_str()),
			}),
			image: raw_coaster.main_image.clone().unwrap().path,
		});
	});

	return Ok(Json(
		join_all(coasters)
			.await
			.into_iter()
			.collect::<Result<Vec<_>, StatusCode>>()?,
	));
}

fn docs_find_coasters_by_user_id(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Get a bucket list")
		.response_with::<200, Json<Vec<BucketList>>, _>(|res| res.description("List of Coasters"))
}

async fn add_coaster(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	Json(coaster_id): Json<u32>,
) -> Result<Json<Vec<u32>>, StatusCode> {
	let store = &store.store;

	#[derive(Deserialize)]
	struct UpdatedBucketList {
		coaster_ids: Vec<u32>,
	}

	let updated: UpdatedBucketList = store
		.update(("bucket_list", user_id))
		.patch(PatchOp::add("/coaster_ids", &[coaster_id]))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
		.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

	return Ok(Json(updated.coaster_ids));
}

fn docs_add_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Add a coaster to a bucket list")
		.response_with::<201, (), _>(|res| res.description("Created bucket list"))
}

async fn insert_coaster(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u32>,
	Path(index): Path<u32>,
	Json(payload): Json<BucketList>,
) -> Result<(), axum::http::StatusCode> {
	todo!()
}

fn docs_insert_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Insert a bucket list")
		.response_with::<200, (), _>(|res| res.description("Inserted bucket list"))
}

async fn set_coasters(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u32>,
	Json(payload): Json<Vec<u32>>,
) -> Result<(), axum::http::StatusCode> {
	todo!()
}

fn docs_set_coasters(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Set a bucket list")
		.response_with::<200, (), _>(|res| res.description("Updated bucket list"))
}

async fn remove_coaster(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u32>,
	Path(index): Path<u32>,
) -> Result<(), axum::http::StatusCode> {
	todo!()
}

fn docs_remove_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Delete a bucket list")
		.response_with::<200, (), _>(|res| res.description("Deleted bucket list"))
}
