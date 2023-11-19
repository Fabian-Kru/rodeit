use aide::{axum::ApiRouter, openapi::OpenApi, transform::TransformOpenApi};
use anyhow::{Ok, Result};
use router::router;
use std::fs::write;

mod router;

#[tokio::main]
async fn main() -> Result<()> {
	let mut api = OpenApi::default();
	let app = ApiRouter::new()
		.nest_api_service("/", router())
		.finish_api_with(&mut api, finish_api);

	// TODO: move to command
	let _ = save_api(&api)?;

	axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
		.serve(app.into_make_service())
		.await?;

	return Ok(());
}

pub fn finish_api(api: TransformOpenApi) -> TransformOpenApi {
	return api.title("Rodeit: Bucket List API").summary("Lorem ipsum");
}

pub fn save_api(api: &OpenApi) -> Result<()> {
	let json = serde_json::to_string_pretty(&api)?;
	write("openapi.json", json)?;
	Ok(())
}
