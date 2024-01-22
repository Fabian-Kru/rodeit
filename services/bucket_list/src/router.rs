use crate::model::{coaster::Coaster, country::Country, manufacturer::Manufacturer, park::Park};
use std::sync::Arc;

use aide::{
	axum::{routing::get_with, ApiRouter},
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
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::opt::PatchOp;

use crate::AppState;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct BucketList {
	coaster_ids: Vec<u32>,
}

pub fn create_router(state: Arc<AppState>) -> ApiRouter {
	ApiRouter::new()
		.api_route(
			"/:user_id",
			get_with(get_coasters, docs_get_coasters)
				.post_with(add_coaster, docs_add_coaster)
				.put_with(set_coasters, docs_set_coasters),
		)
		.api_route(
			"/:user_id/:index",
			get_with(get_coaster, docs_get_coaster)
				.post_with(insert_coaster, docs_insert_coaster)
				.delete_with(delete_coaster, docs_delete_coaster),
		)
		.with_state(state)
}

async fn get_coasters(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
) -> Result<Json<Vec<Coaster>>, StatusCode> {
	let store = &state.store;

	let bucket_list: Option<BucketList> = store
		.select(("bucket_list", user_id))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let bucket_list = bucket_list.ok_or(StatusCode::NOT_FOUND)?;

	let coasters = bucket_list.coaster_ids.iter().map(|coaster_id| async {
		let raw_coaster = CoasterReadCoaster::get_coaster_item()
			.id(coaster_id.to_string())
			.send(&state.cc_client)
			.await
			.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

		return Ok(Coaster {
			id: *coaster_id,
			name: raw_coaster.name.clone(),
			speed: raw_coaster.speed.and_then(|speed| Some(speed as u32)),
			height: raw_coaster.height.and_then(|height| Some(height as u32)),
			inversions: raw_coaster
				.inversions_number
				.and_then(|inversions| Some(inversions as u32)),
			manufacturer: raw_coaster.manufacturer.clone().and_then(|manufacturer| {
				Some(Manufacturer {
					name: manufacturer.name?,
				})
			}),
			park: raw_coaster.park.clone().and_then(|park| {
				Some(Park {
					id: park.id,
					name: park.name?,
					country: park
						.country
						.and_then(|country| Country::from_id(country.name?.as_str())),
				})
			}),
			image: raw_coaster.main_image.clone().and_then(|image| image.path),
		});
	});

	return Ok(Json(
		join_all(coasters)
			.await
			.into_iter()
			.collect::<Result<Vec<_>, StatusCode>>()?,
	));
}

fn docs_get_coasters(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Get all coasters in a bucket list")
		.response_with::<200, Json<Vec<Coaster>>, _>(|res| {
			res.description("List of Coasters in bucket list")
		})
}

async fn get_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
) -> Result<Json<Coaster>, StatusCode> {
	let mut result = state.store
		.query(
			"SELECT VALUE array::at(coaster_ids, $index) FROM ONLY type::thing('bucket_list', $user_id);",
		)
		.bind(("index", index))
		.bind(("user_id", user_id))
		.await
			.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let coaster_id = result
		.take::<Option<u64>>(0)
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
		.ok_or(StatusCode::NOT_FOUND)?;

	let raw_coaster = CoasterReadCoaster::get_coaster_item()
		.id(coaster_id.to_string())
		.send(&state.cc_client)
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	return Ok(Json(Coaster {
		id: coaster_id as u32,
		name: raw_coaster.name.clone(),
		speed: raw_coaster.speed.and_then(|speed| Some(speed as u32)),
		height: raw_coaster.height.and_then(|height| Some(height as u32)),
		inversions: raw_coaster
			.inversions_number
			.and_then(|inversions| Some(inversions as u32)),
		manufacturer: raw_coaster.manufacturer.clone().and_then(|manufacturer| {
			Some(Manufacturer {
				name: manufacturer.name?,
			})
		}),
		park: raw_coaster.park.clone().and_then(|park| {
			Some(Park {
				id: park.id,
				name: park.name?,
				country: park
					.country
					.and_then(|country| Country::from_id(country.name?.as_str())),
			})
		}),
		image: raw_coaster.main_image.clone().and_then(|image| image.path),
	}));
}

fn docs_get_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Get a coaster at a given index in a bucket list")
		.response_with::<200, Json<Coaster>, _>(|res| res.description("Coaster in bucket list"))
}

async fn add_coaster(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	Json(coaster_id): Json<u32>,
) -> Result<StatusCode, StatusCode> {
	let updated: Option<BucketList> = state
		.store
		.update(("bucket_list", user_id))
		.patch(PatchOp::add("/coaster_ids", &[coaster_id]))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	return Ok(StatusCode::OK);
}

fn docs_add_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Add a coaster to a bucket list")
		.response_with::<200, (), _>(|res| res.description("Added to bucket list"))
}

async fn insert_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
	Json(coaster_id): Json<u32>,
) -> Result<StatusCode, StatusCode> {
	let mut result = state.store.query("UPDATE type::thing('bucket_list', $user_id) SET coaster_ids = array::insert(coaster_ids, $coaster_id, $index);")
        .bind(("user_id", user_id))
        .bind(("index", index))
        .bind(("coaster_id", coaster_id))
        .await.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let updated: Option<BucketList> = result.take(0).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	return Ok(StatusCode::OK);
}

fn docs_insert_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Insert a coaster at a given index into a bucket list")
		.response_with::<200, (), _>(|res| res.description("Inserted into bucket list"))
}

async fn set_coasters(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	Json(coaster_ids): Json<Vec<u32>>,
) -> Result<StatusCode, StatusCode> {
	let updated: Option<BucketList> = state
		.store
		.update(("bucket_list", user_id))
		.content(BucketList { coaster_ids })
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	return Ok(StatusCode::OK);
}

fn docs_set_coasters(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Set all coasters in a bucket list")
		.response_with::<200, (), _>(|res| res.description("Set bucket list"))
}

async fn delete_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
) -> Result<StatusCode, StatusCode> {
	let updated: Option<BucketList> = state
		.store
		.update(("bucket_list", user_id))
		.patch(PatchOp::remove(format!("/coaster_ids/{}", index).as_str()))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	return Ok(StatusCode::OK);
}

fn docs_delete_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Delete a coaster at a given index from a bucket list")
		.response_with::<200, (), _>(|res| res.description("Deleted from bucket list"))
}
