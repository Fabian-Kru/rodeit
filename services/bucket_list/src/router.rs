use std::sync::Arc;

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
use captain_coaster::apis::coaster_api;
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
	country: String,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
struct Manufacturer {
	id: u32,
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
		let raw_coaster =
			coaster_api::get_coaster_item(&state.captain_coaster_config, &coaster_id.to_string())
				.await
				.unwrap();

		return Coaster {
			id: *coaster_id,
			name: raw_coaster.name,
			speed: raw_coaster.speed.and_then(|speed| Some(speed as u32)),
			height: raw_coaster.height.and_then(|height| Some(height as u32)),
			inversions: raw_coaster
				.inversions_number
				.and_then(|inversions| Some(inversions as u32)),
			manufacturer: raw_coaster.manufacturer.map(|manufacturer| Manufacturer {
				id: 0u32, // TODO: get manufacturer id
				name: manufacturer.name.unwrap(),
			}),
			park: raw_coaster.park.map(|park| Park {
				id: 0u32, // TODO: get manufacturer id
				name: park.name.unwrap(),
				country: park.country.unwrap().name.unwrap(), // TODO: unwrap city
			}),
			image: raw_coaster.main_image.unwrap().path,
		};
	});

	return Ok(Json(join_all(coasters).await));
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
