use std::sync::Arc;

use aide::{
	axum::{
		routing::{get_with, post_with},
		ApiRouter,
	},
	transform::TransformOperation,
};
use axum::{
	extract::{Path, State},
	Json,
};
use captain_coaster::apis::coaster_api;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::AppState;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct BucketList {
	user_id: u64,
	coaster_ids: Vec<u64>,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
struct Park {
	id: u64,
	name: String,
	country: String,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
struct Manufacturer {
	id: u64,
	name: String,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, JsonSchema)]
struct Coaster {
	id: i32,
	name: String,
	speed: Option<i32>,
	height: Option<i32>,
	inversions: Option<i32>,
	manufacturer: Option<Manufacturer>,
	park: Option<Park>,
	image: Option<String>,
}

pub fn create_router(state: Arc<AppState>) -> ApiRouter {
	ApiRouter::new()
		.api_route(
			"/:user_id",
			get_with(find, find_docs)
				.post_with(append, append_docs)
				.put_with(set, set_docs),
		)
		.api_route(
			"/:user_id/:index",
			post_with(insert, insert_docs).delete_with(delete, delete_docs),
		)
		.with_state(state)
}

async fn find(State(state): State<Arc<AppState>>, Path(user_id): Path<u64>) -> Json<Vec<Coaster>> {
	let coasters = coaster_api::get_coaster_collection(
		&state.captain_coaster_config,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
		None,
	)
	.await
	.unwrap_or_else(|res| {
		dbg!(res);
		return vec![];
	});

	let n = coasters
		.iter()
		.map(|coaster| Coaster {
			id: coaster.id.unwrap_or(0),
			name: coaster.name.clone(),
			speed: None,
			height: None,
			inversions: None,
			manufacturer: None,
			park: None,
			image: None,
		})
		.collect::<Vec<Coaster>>()
		.to_vec();

	return Json(n);
}

fn find_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Get a bucket list")
		.response_with::<200, Json<Vec<BucketList>>, _>(|res| res.description("List of Coasters"))
}

async fn append(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	Json(payload): Json<BucketList>,
) -> Result<(), axum::http::StatusCode> {
	todo!()
}

fn append_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Create a new bucket list")
		.response_with::<201, (), _>(|res| res.description("Created bucket list"))
}

async fn insert(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	Path(index): Path<u8>,
	Json(payload): Json<BucketList>,
) -> Result<(), axum::http::StatusCode> {
	todo!()
}

fn insert_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Insert a bucket list")
		.response_with::<200, (), _>(|res| res.description("Inserted bucket list"))
}

async fn set(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	Json(payload): Json<BucketList>,
) -> Result<(), axum::http::StatusCode> {
	todo!()
}

fn set_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Set a bucket list")
		.response_with::<200, (), _>(|res| res.description("Updated bucket list"))
}

async fn delete(
	State(store): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	Path(index): Path<u8>,
) -> Result<(), axum::http::StatusCode> {
	todo!()
}

fn delete_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Delete a bucket list")
		.response_with::<200, (), _>(|res| res.description("Deleted bucket list"))
}
