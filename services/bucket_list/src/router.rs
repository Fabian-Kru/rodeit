use crate::model::{
	coaster::{Coaster, CoasterAndBucketListCount},
	country::Country,
	manufacturer::Manufacturer,
	park::Park,
};
use std::sync::Arc;

use aide::{
	axum::{routing::get_with, ApiRouter},
	openapi::OpenApi,
	scalar::Scalar,
	transform::{TransformOpenApi, TransformOperation},
};
use anyhow::Result;
use auth::Claims;
use axum::{
	extract::{Path, State},
	http::StatusCode,
	routing::get,
	Extension, Json, Router,
};
use captain_coaster::{client::Sendable, coaster_read_coaster::CoasterReadCoaster};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use surrealdb::opt::PatchOp;

use crate::AppState;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct BucketList {
	coaster_ids: Vec<u32>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
	let mut api = OpenApi::default();
	ApiRouter::new()
		.route("/bucket_list/openapi.json", get(get_openapi))
		.route(
			"/bucket_list/docs",
			get(Scalar::new("/openapi.json").axum_handler()),
		)
		.api_route(
			"/bucket_list/",
			get_with(
				get_coasters_and_bucket_list_counts,
				docs_get_coasters_and_bucket_list_counts,
			),
		)
		.api_route(
			"/bucket_list/:user_id",
			get_with(get_coasters, docs_get_coasters)
				.post_with(add_coaster, docs_add_coaster)
				.put_with(set_coasters, docs_set_coasters),
		)
		.api_route(
			"/bucket_list/:user_id/:index",
			get_with(get_coaster, docs_get_coaster)
				.post_with(insert_coaster, docs_insert_coaster)
				.delete_with(delete_coaster, docs_delete_coaster),
		)
		.finish_api_with(&mut api, openapi)
		.layer(Extension(Arc::new(api)))
		.with_state(state)
}

fn openapi(openapi: TransformOpenApi) -> TransformOpenApi {
	return openapi
		.title("Bucket List API")
		.version(env!("CARGO_PKG_VERSION"))
		.security_scheme(
			"jwt",
			aide::openapi::SecurityScheme::Http {
				scheme: "bearer".to_string(),
				bearer_format: Some("JWT".to_string()),
				description: None,
				extensions: Default::default(),
			},
		);
}

async fn get_openapi(Extension(api): Extension<Arc<OpenApi>>) -> Json<Arc<OpenApi>> {
	return Json(api);
}

async fn get_coasters_and_bucket_list_counts(
	State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CoasterAndBucketListCount>>, StatusCode> {
	let store = &state.store;

	let mut result = store
		.query("SELECT count(), coaster_ids AS coaster_id FROM bucket_list SPLIT coaster_id GROUP BY coaster_id ORDER BY count DESC;")
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	#[derive(Debug, Deserialize)]
	struct CoasterIdAndBucketListCount {
		coaster_id: u32,
		count: u32,
	}

	let coaster_ids_and_bucket_list_counts = result
		.take::<Vec<CoasterIdAndBucketListCount>>(0)
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let coasters_and_bucket_list_counts =
		coaster_ids_and_bucket_list_counts
			.iter()
			.map(|coaster_with_count| async {
				let raw_coaster = CoasterReadCoaster::get_coaster_item()
					.id(coaster_with_count.coaster_id.to_string())
					.send(&state.cc_client)
					.await
					.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

				return Ok(CoasterAndBucketListCount {
					coaster: Coaster {
						id: coaster_with_count.coaster_id,
						name: raw_coaster.name.clone(),
						speed: raw_coaster.speed.and_then(|speed| Some(speed as u32)),
						height: raw_coaster.height.and_then(|height| Some(height as u32)),
						length: raw_coaster.length.and_then(|length| Some(length as u32)),
						inversions: raw_coaster
							.inversions_number
							.and_then(|inversions| Some(inversions as u32)),
						manufacturer: raw_coaster.manufacturer.clone().and_then(|manufacturer| {
							Some(Manufacturer {
								name: manufacturer.name?,
							})
						}),
						park: raw_coaster.park.clone().and_then(|park| {
							Some(Park {
								id: park.id,
								name: park.name?,
								country: park
									.country
									.and_then(|country| Country::from_id(country.name?.as_str())),
							})
						}),
						image: raw_coaster.main_image.clone().and_then(|image| image.path),
					},
					bucket_list_count: coaster_with_count.count,
				});
			});

	return Ok(Json(
		join_all(coasters_and_bucket_list_counts)
			.await
			.into_iter()
			.collect::<Result<Vec<CoasterAndBucketListCount>, StatusCode>>()?,
	));
}

fn docs_get_coasters_and_bucket_list_counts(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Get Coasters and Bucket List Counts")
		.description("Get all coasters and the amount of bucket lists they are in")
		.response_with::<200, Json<Vec<CoasterAndBucketListCount>>, _>(|res| {
			res.description("List of Coasters and Bucket List Counts")
				.example(vec![
					CoasterAndBucketListCount {
						bucket_list_count: 12,
						coaster: Coaster {
							id: 2832,
							name: "Zadra".to_string(),
							speed: Some(121),
							height: Some(63),
							length: Some(1316),
							inversions: Some(3),
							manufacturer: Some(Manufacturer {
								name: "Rocky Mountain Construction".to_string(),
							}),
							park: Some(Park {
								id: 545,
								name: "Energylandia".to_string(),
								country: Some(Country::Poland),
							}),
							image: Some("9f68e5f6-f989-4f0d-a9f8-1330dad339e3.jpg".to_string()),
						},
					},
					CoasterAndBucketListCount {
						bucket_list_count: 8,
						coaster: Coaster {
							id: 2827,
							name: "Taiga".to_string(),
							speed: Some(106),
							height: Some(52),
							length: Some(1104),
							inversions: Some(4),
							manufacturer: Some(Manufacturer {
								name: "Intamin".to_string(),
							}),
							park: Some(Park {
								id: 117,
								name: "Linnanmäki".to_string(),
								country: Some(Country::Finland),
							}),
							image: Some("9a6ed72f-34c7-4353-bcf5-49fbae03718b.jpeg".to_string()),
						},
					},
				])
		})
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn get_coasters(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
) -> Result<Json<Vec<Coaster>>, StatusCode> {
	let store = &state.store;

	let bucket_list: Option<BucketList> = store
		.select(("bucket_list", user_id))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let bucket_list = bucket_list.ok_or(StatusCode::NOT_FOUND)?;

	let coasters = bucket_list.coaster_ids.iter().map(|coaster_id| async {
		let raw_coaster = CoasterReadCoaster::get_coaster_item()
			.id(coaster_id.to_string())
			.send(&state.cc_client)
			.await
			.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

		return Ok(Coaster {
			id: *coaster_id,
			name: raw_coaster.name.clone(),
			speed: raw_coaster.speed.and_then(|speed| Some(speed as u32)),
			height: raw_coaster.height.and_then(|height| Some(height as u32)),
			length: raw_coaster.length.and_then(|length| Some(length as u32)),
			inversions: raw_coaster
				.inversions_number
				.and_then(|inversions| Some(inversions as u32)),
			manufacturer: raw_coaster.manufacturer.clone().and_then(|manufacturer| {
				Some(Manufacturer {
					name: manufacturer.name?,
				})
			}),
			park: raw_coaster.park.clone().and_then(|park| {
				Some(Park {
					id: park.id,
					name: park.name?,
					country: park
						.country
						.and_then(|country| Country::from_id(country.name?.as_str())),
				})
			}),
			image: raw_coaster.main_image.clone().and_then(|image| image.path),
		});
	});

	return Ok(Json(
		join_all(coasters)
			.await
			.into_iter()
			.collect::<Result<Vec<_>, StatusCode>>()?,
	));
}

fn docs_get_coasters(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Get Coasters in a Bucket List")
		.description("Get all coasters in a bucket list")
		.response_with::<200, Json<Vec<Coaster>>, _>(|res| {
			res.description("List of Coasters in Bucket List")
				.example(vec![
					Coaster {
						id: 2832,
						name: "Zadra".to_string(),
						speed: Some(121),
						height: Some(63),
						length: Some(1316),
						inversions: Some(3),
						manufacturer: Some(Manufacturer {
							name: "Rocky Mountain Construction".to_string(),
						}),
						park: Some(Park {
							id: 545,
							name: "Energylandia".to_string(),
							country: Some(Country::Poland),
						}),
						image: Some("9f68e5f6-f989-4f0d-a9f8-1330dad339e3.jpg".to_string()),
					},
					Coaster {
						id: 2827,
						name: "Taiga".to_string(),
						speed: Some(106),
						height: Some(52),
						length: Some(1104),
						inversions: Some(4),
						manufacturer: Some(Manufacturer {
							name: "Intamin".to_string(),
						}),
						park: Some(Park {
							id: 117,
							name: "Linnanmäki".to_string(),
							country: Some(Country::Finland),
						}),
						image: Some("9a6ed72f-34c7-4353-bcf5-49fbae03718b.jpeg".to_string()),
					},
				])
		})
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn get_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
) -> Result<Json<Coaster>, StatusCode> {
	let mut result = state.store
		.query(
			"SELECT VALUE array::at(coaster_ids, $index) FROM ONLY type::thing('bucket_list', $user_id);",
		)
		.bind(("index", index))
		.bind(("user_id", user_id))
		.await
			.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let coaster_id = result
		.take::<Option<u64>>(0)
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
		.ok_or(StatusCode::NOT_FOUND)?;

	let raw_coaster = CoasterReadCoaster::get_coaster_item()
		.id(coaster_id.to_string())
		.send(&state.cc_client)
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	return Ok(Json(Coaster {
		id: coaster_id as u32,
		name: raw_coaster.name.clone(),
		speed: raw_coaster.speed.and_then(|speed| Some(speed as u32)),
		height: raw_coaster.height.and_then(|height| Some(height as u32)),
		length: raw_coaster.length.and_then(|length| Some(length as u32)),
		inversions: raw_coaster
			.inversions_number
			.and_then(|inversions| Some(inversions as u32)),
		manufacturer: raw_coaster.manufacturer.clone().and_then(|manufacturer| {
			Some(Manufacturer {
				name: manufacturer.name?,
			})
		}),
		park: raw_coaster.park.clone().and_then(|park| {
			Some(Park {
				id: park.id,
				name: park.name?,
				country: park
					.country
					.and_then(|country| Country::from_id(country.name?.as_str())),
			})
		}),
		image: raw_coaster.main_image.clone().and_then(|image| image.path),
	}));
}

fn docs_get_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Get a Coaster by index")
		.description("Get a coaster at a given index in a bucket list")
		.response_with::<200, Json<Coaster>, _>(|res| {
			res.description("Coaster in Bucket List").example(Coaster {
				id: 2827,
				name: "Taiga".to_string(),
				speed: Some(106),
				height: Some(52),
				length: Some(1104),
				inversions: Some(4),
				manufacturer: Some(Manufacturer {
					name: "Intamin".to_string(),
				}),
				park: Some(Park {
					id: 117,
					name: "Linnanmäki".to_string(),
					country: Some(Country::Finland),
				}),
				image: Some("9a6ed72f-34c7-4353-bcf5-49fbae03718b.jpeg".to_string()),
			})
		})
		.response_with::<404, (), _>(|res| {
			res.description("Bucket List not found or index out of bounds")
		})
}

async fn add_coaster(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	claims: Claims,
	Json(coaster_id): Json<u32>,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let updated: Option<BucketList> = state
		.store
		.update(("bucket_list", user_id))
		.patch(PatchOp::add("/coaster_ids", &[coaster_id]))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	return Ok(StatusCode::OK);
}

fn docs_add_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Add a Coaster")
		.description("Add a coaster to a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Added to Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn insert_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
	claims: Claims,
	Json(coaster_id): Json<u32>,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let mut result = state.store.query("UPDATE type::thing('bucket_list', $user_id) SET coaster_ids = array::insert(coaster_ids, $coaster_id, $index);")
        .bind(("user_id", user_id))
        .bind(("index", index))
        .bind(("coaster_id", coaster_id))
        .await.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	let updated: Option<BucketList> = result.take(0).or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	return Ok(StatusCode::OK);
}

fn docs_insert_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Insert a Coaster by index")
		.description("Insert a coaster at a given index into a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Inserted into Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn set_coasters(
	State(state): State<Arc<AppState>>,
	Path(user_id): Path<u64>,
	claims: Claims,
	Json(coaster_ids): Json<Vec<u32>>,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let updated: Option<BucketList> = state
		.store
		.update(("bucket_list", user_id))
		.content(BucketList { coaster_ids })
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	return Ok(StatusCode::OK);
}

fn docs_set_coasters(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Set Coasters in a Bucket List")
		.description("Set all coasters in a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Set Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| res.description("Bucket List not found"))
}

async fn delete_coaster(
	State(state): State<Arc<AppState>>,
	Path((user_id, index)): Path<(u64, usize)>,
	claims: Claims,
) -> Result<StatusCode, StatusCode> {
	if claims.sub != user_id.to_string() {
		return Err(StatusCode::UNAUTHORIZED);
	}

	let updated: Option<BucketList> = state
		.store
		.update(("bucket_list", user_id))
		.patch(PatchOp::remove(format!("/coaster_ids/{}", index).as_str()))
		.await
		.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;

	if updated.is_none() {
		return Err(StatusCode::NOT_FOUND);
	}

	// BUG: This still returns 200 if the index is out of bounds
	return Ok(StatusCode::OK);
}

fn docs_delete_coaster(operation: TransformOperation) -> TransformOperation {
	operation
		.summary("Delete a Coaster by index")
		.description("Delete a coaster at a given index from a bucket list")
		.security_requirement("jwt")
		.response_with::<200, (), _>(|res| res.description("Deleted from Bucket List"))
		.response_with::<401, (), _>(|res| res.description("Unauthorized"))
		.response_with::<404, (), _>(|res| {
			res.description("Bucket List not found or index out of bounds")
		})
}
