mod record;

use std::sync::Arc;
use aide::{
    axum::ApiRouter,
    openapi::OpenApi,
    transform::TransformOpenApi,
};
use aide::axum::routing::get;
use axum::Extension;
use axum::response::Html;

#[tokio::main]
async fn main() {
    aide::gen::extract_schemas(true);

    let mut api = OpenApi::default();

    let app = ApiRouter::new()
        .route("/", get(handler))
        .finish_api_with(&mut api, api_docs)
        .layer(Extension(Arc::new(api)));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("RodeIt Credit Service")
}

async fn handler() -> Html<& 'static str> {
    Html("Hello, World!")
}