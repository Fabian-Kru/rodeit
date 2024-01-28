use std::{env::current_dir, sync::Arc};

use anyhow::{Ok, Result};
use router::create_router;
use surrealdb::Surreal;

mod router;

pub struct AppState {
	store: Surreal<surrealdb::engine::local::Db>,
}

const DATABASE_FILENAME: &str = "bucket_list.db";
const DATABASE_NAMESPACE: &str = "rodeit";
const DATABASE_NAME: &str = "bucket_list";

#[tokio::main]
async fn main() -> Result<()> {
	let db = Surreal::new::<surrealdb::engine::local::File>(current_dir()?.join(DATABASE_FILENAME))
		.await?;
	db.use_ns(DATABASE_NAMESPACE).use_db(DATABASE_NAME).await?;

	let app_state = Arc::new(AppState { store: db });

	let router = create_router(app_state);
	let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
	axum::serve(listener, router).await.unwrap();

	return Ok(());
}
