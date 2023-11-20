use anyhow::{Ok, Result};
use router::create_router;

mod router;

#[tokio::main]
async fn main() -> Result<()> {
	axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
		.serve(create_router().into_make_service())
		.await?;

	return Ok(());
}
