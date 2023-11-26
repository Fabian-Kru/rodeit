use std::{env::current_dir, sync::Arc};

use anyhow::{Ok, Result};
use router::create_router;
use surrealdb::Surreal;

pub mod router;

pub struct AppState {
	store: Surreal<surrealdb::engine::local::Db>,
	captain_coaster_config: captain_coaster::apis::configuration::Configuration,
}

const DATABASE_FILENAME: &str = "bucket_list.db";
const DATABASE_NAMESPACE: &str = "rodeit";
const DATABASE_NAME: &str = "bucket_list";

#[tokio::main]
async fn main() -> Result<()> {
	let db = Surreal::new::<surrealdb::engine::local::File>(current_dir()?.join(DATABASE_FILENAME))
		.await?;
	db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await?;

	let mut captain_coaster_default_headers = reqwest::header::HeaderMap::new();
	captain_coaster_default_headers.insert(
		"Accept",
		reqwest::header::HeaderValue::from_static("application/json"),
	);

	let captain_coaster_config = captain_coaster::apis::configuration::Configuration {
		client: reqwest::ClientBuilder::new()
			.default_headers(captain_coaster_default_headers)
			.build()?,
		base_path: "https://captaincoaster.com".into(),
		user_agent: Some("HTTPie/3.2.2".into()),
		basic_auth: None,
		oauth_access_token: None,
		bearer_access_token: None,
		api_key: Some(captain_coaster::apis::configuration::ApiKey {
			prefix: Some("Bearer".into()),
			key: ("ee2d7ceb-eabe-4583-8b58-be70ff1ab79e".into()),
		}),
	};

	let app_state = Arc::new(AppState {
		store: db,
		captain_coaster_config,
	});

	axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
		.serve(create_router(app_state).into_make_service())
		.await?;

	return Ok(());
}
