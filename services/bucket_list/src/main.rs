use std::{env::current_dir, sync::Arc};

use anyhow::{Ok, Result};
use router::create_router;
use surrealdb::{engine::local::File, Surreal};

pub mod router;

#[tokio::main]
async fn main() -> Result<()> {
	let db = Surreal::new::<File>(current_dir()?.join("bucket_list.db")).await?;
	db.use_ns("rodeit").use_db("bucket_list").await?;

	axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
		.serve(create_router(Arc::new(db)).into_make_service())
		.await?;

	return Ok(());
}
