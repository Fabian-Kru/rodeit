use std::sync::Arc;

use aide::{
	axum::{routing::get_with, ApiRouter, IntoApiResponse},
	transform::TransformOperation,
};
use axum::{extract::State, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use surrealdb::{Connection, Surreal};

pub fn create_router(store: Arc<Surreal<impl Connection>>) -> ApiRouter {
	ApiRouter::new()
		.api_route(
			"/",
			get_with(bucket_lists, bucket_lists_docs)
				.post_with(create_bucket_list, create_bucket_list_docs),
		)
		.with_state(store)
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct BucketList {
	id: u64,
	name: String,
}

async fn bucket_lists() -> impl IntoApiResponse {}

fn bucket_lists_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Find all bucket lists")
		.response_with::<200, Json<Vec<BucketList>>, _>(|res| {
			res.description("List of bucket lists")
		})
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
struct CreateBucketList {
	name: String,
}

async fn create_bucket_list(
	State(store): State<Arc<Surreal<impl Connection>>>,
	Json(payload): Json<CreateBucketList>,
) -> Result<(), axum::http::StatusCode> {
	let created: Vec<CreateBucketList> = store
		.create("bucket_list")
		.content(CreateBucketList { name: payload.name })
		.await
		.or(Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR))?;

	dbg!(created);

	Ok(())
}

fn create_bucket_list_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Create a new bucket list")
		.response_with::<201, (), _>(|res| res.description("Created bucket list"))
}
