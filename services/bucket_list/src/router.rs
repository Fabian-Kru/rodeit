use aide::{
	axum::{routing::get_with, ApiRouter},
	transform::TransformOperation,
};
use axum::Json;
use schemars::JsonSchema;
use serde::Serialize;

pub fn create_router() -> ApiRouter {
	ApiRouter::new().api_route(
		"/",
		get_with(bucket_lists, bucket_lists_docs)
			.post_with(create_bucket_list, create_bucket_list_docs),
	)
}

#[derive(Serialize, JsonSchema)]
struct BucketList {
	id: u64,
	name: String,
}

async fn bucket_lists() {}

fn bucket_lists_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Find all bucket lists")
		.response_with::<200, Json<Vec<BucketList>>, _>(|res| {
			res.description("List of bucket lists")
		})
}

async fn create_bucket_list() {}

fn create_bucket_list_docs(operation: TransformOperation) -> TransformOperation {
	operation
		.description("Create a new bucket list")
		.response_with::<201, (), _>(|res| res.description("Created bucket list"))
}
