use std::{env::current_dir, sync::Arc};

use anyhow::{Ok, Result};
use reqwest::header::HeaderValue;
use reqwest::Client;
use router::create_router;
use surrealdb::Surreal;

mod auth;
mod model;
mod router;

pub struct AppState {
	store: Surreal<surrealdb::engine::local::Db>,
	cc_client: Client,
}

const DATABASE_FILENAME: &str = "bucket_list.db";
const DATABASE_NAMESPACE: &str = "rodeit";
const DATABASE_NAME: &str = "bucket_list";

#[tokio::main]
async fn main() -> Result<()> {
	let db = Surreal::new::<surrealdb::engine::local::File>(current_dir()?.join(DATABASE_FILENAME))
		.await?;
	db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await?;

	let mut cc_default_headers = reqwest::header::HeaderMap::new();
	cc_default_headers.insert("Accept", HeaderValue::from_static("application/ld+json"));
	cc_default_headers.insert(
		"Authorization",
		HeaderValue::from_static("Bearer ee2d7ceb-eabe-4583-8b58-be70ff1ab79e"),
	);

	let cc_client = Client::builder()
		.default_headers(cc_default_headers)
		.build()?;
	let app_state = Arc::new(AppState {
		store: db,
		cc_client,
	});

	let router = create_router(app_state);
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, router).await.unwrap();

	return Ok(());
}
