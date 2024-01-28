use std::sync::Arc;

use aide::{
	axum::{routing::get_with, ApiRouter},
	openapi::OpenApi,
	scalar::Scalar,
	transform::{TransformOpenApi, TransformOperation},
};
use anyhow::Result;
use auth::Claims;
use axum::{
	extract::{Path, State},
	http::StatusCode,
	routing::get,
	Extension, Json, Router,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::opt::PatchOp;

use crate::AppState;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct BucketList {
	coaster_ids: Vec<u64>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
	let mut api = OpenApi::default();
	ApiRouter::new()
		.route("/openapi.json", get(get_openapi))
		.route("/docs", get(Scalar::new("/openapi.json").axum_handler()))
		.api_route(
			"/",
			get_with(get_coasters_and_counts, docs_get_coasters_and_counts),
		)
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
		.finish_api_with(&mut api, openapi)
		.layer(Extension(Arc::new(api)))
		.with_state(state)
}

fn openapi(openapi: TransformOpenApi) -> TransformOpenApi {
	return openapi
		.title("Bucket List API")
		.version(env!("CARGO_PKG_VERSION"))
		.security_scheme(
			"jwt",
			aide::openapi::SecurityScheme::Http {
				scheme: "bearer".to_string(),
				bearer_format: Some("JWT".to_string()),
				description: None,
				extensions: Default::default(),
			},
		);
}

async fn get_openapi(Extension(api): Extension<Arc<OpenApi>>) -> Json<Arc<OpenApi>> {
	return Json(api);
}

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct CoasterIdAndCount {
	pub coaster_id: u64,
	pub count: u32,
}

async fn get_coasters_and_counts(
	State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CoasterIdAndCount>>, StatusCode> {
	let store = &state.store;

	let mut result = store
		.query("SELECT count(), coaster_ids AS coaster_id FROM bucket_list SPLIT coaster_id GROUP BY coaster_id ORDER BY count DESC;")
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let coaster_ids_and_counts = result
		.take::<Vec<CoasterIdAndCount>>(0)
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	return Ok(Json(coaster_ids_and_counts));
}

fn docs_get_coasters_and_counts(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Get Coasters and Bucket List Counts")
		.description("Get all coasters and the amount of bucket lists they are in")
		.response_with::<200, Json<Vec<CoasterIdAndCount>>, _>(|res| {
			res.description("List of Coasters and Bucket List Counts")
				.example(vec![
					CoasterIdAndCount {
						count: 12,
						coaster_id: 2832,
					},
					CoasterIdAndCount {
						count: 8,
						coaster_id: 2827,
					},
				])
		})
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn get_coasters(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
) -> Result<Json<Vec<u64>>, StatusCode> {
	let store = &state.store;

	let bucket_list: Option<BucketList> = store
		.select(("bucket_list", user_id))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let bucket_list = bucket_list.ok_or(StatusCode::NOT_FOUND)?;

	return Ok(Json(bucket_list.coaster_ids));
}

fn docs_get_coasters(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Get Coasters in a Bucket List")
		.description("Get all coasters in a bucket list")
		.response_with::<200, Json<Vec<u64>>, _>(|res| {
			res.description("List of Coasters in Bucket List")
				.example(vec![2832, 2827])
		})
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn get_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
) -> Result<Json<u64>, (StatusCode, String)> {
	let mut result = state.store
		.query(
			"SELECT VALUE array::at(coaster_ids, $index) FROM ONLY type::thing('bucket_list', $user_id);",
		)
		.bind(("index", index))
		.bind(("user_id", user_id))
		.await
			.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

	let coaster_id = result
		.take::<Option<u64>>(0)
		.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
		.ok_or((StatusCode::NOT_FOUND, "not found".to_string()))?;

	return Ok(Json(coaster_id));
}

fn docs_get_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Get a Coaster by index")
		.description("Get a coaster at a given index in a bucket list")
		.response_with::<200, Json<u64>, _>(|res| {
			res.description("Coaster in Bucket List").example(2827u32)
		})
		.response_with::<404, (), _>(|res| {
			res.description("Bucket List not found or index out of bounds")
		})
}

async fn add_coaster(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	claims: Claims,
	Json(coaster_id): Json<u32>,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

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
		.summary("Add a Coaster")
		.description("Add a coaster to a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Added to Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn insert_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
	claims: Claims,
	Json(coaster_id): Json<u32>,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

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
		.summary("Insert a Coaster by index")
		.description("Insert a coaster at a given index into a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Inserted into Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn set_coasters(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	claims: Claims,
	Json(coaster_ids): Json<Vec<u64>>,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

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
		.summary("Set Coasters in a Bucket List")
		.description("Set all coasters in a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Set Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn delete_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
	claims: Claims,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let updated: Option<BucketList> = state
		.store
		.update(("bucket_list", user_id))
		.patch(PatchOp::remove(format!("/coaster_ids/{}", index).as_str()))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	// BUG: This still returns 200 if the index is out of bounds
	return Ok(StatusCode::OK);
}

fn docs_delete_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Delete a Coaster by index")
		.description("Delete a coaster at a given index from a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Deleted from Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| {
			res.description("Bucket List not found or index out of bounds")
		})
}
