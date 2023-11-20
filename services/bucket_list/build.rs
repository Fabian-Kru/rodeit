use aide::{openapi::OpenApi, transform::TransformOpenApi};
use anyhow::{anyhow, Error};
use router::create_router;
use std::env::var;
use std::fs::write;
use std::path::PathBuf;

#[path = "src/router.rs"]
mod router;

const FILENAME: &str = "openapi.json";

fn main() {
	let mut api = OpenApi::default();
	let _ = create_router().finish_api_with(&mut api, finish_api);
	save_api(&api).unwrap();
}

fn save_api(api: &OpenApi) -> Result<(), Error> {
	let json = serde_json::to_string_pretty(&api)?;
	write(cargo_target_dir()?.join(FILENAME), json)?;
	Ok(())
}

fn finish_api(api: TransformOpenApi) -> TransformOpenApi {
	return api.title("Rodeit: Bucket List API").summary("Lorem ipsum");
}

// https://github.com/rust-lang/cargo/issues/9661#issuecomment-1812847609
fn cargo_target_dir() -> Result<PathBuf, Error> {
	let skip_triple = var("TARGET")? == var("HOST")?;
	let skip_parent_dirs = if skip_triple { 3 } else { 4 };

	let out_dir = PathBuf::from(var("OUT_DIR")?);
	let mut current = out_dir.as_path();
	for _ in 0..skip_parent_dirs {
		current = current.parent().ok_or(anyhow!("not found"))?;
	}

	Ok(PathBuf::from(current))
}
