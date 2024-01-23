use std::env;

use aide::OperationIo;
use axum::{
	async_trait,
	extract::FromRequestParts,
	http::{request::Parts, StatusCode},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

const ALGORITHM: Algorithm = Algorithm::HS256;

#[derive(Debug, Serialize, Deserialize, OperationIo)]
pub struct Claims {
	pub sub: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
		let secret = env::var("RODEIT_SECRET").or(Err((
			StatusCode::INTERNAL_SERVER_ERROR,
			"Could not load encryption secret",
		)))?;
		let authorization_header = parts
			.headers
			.get("Authorization")
			.ok_or((
				StatusCode::UNAUTHORIZED,
				"`Authorization` header is missing",
			))?
			.to_str()
			.or(Err((
				StatusCode::BAD_REQUEST,
				"`Authorization` header needs to be a string",
			)))?;
		let token = authorization_header.strip_prefix("Bearer ").ok_or((
			StatusCode::BAD_REQUEST,
			"Authentication must be a Bearer token containing a JWT",
		))?;
		let jwt = jsonwebtoken::decode::<Claims>(
			token,
			&DecodingKey::from_secret(secret.as_bytes()),
			&Validation::new(ALGORITHM),
		)
		.or(Err((StatusCode::BAD_REQUEST, "Invalid JWT")))?;

		return Ok(jwt.claims);
	}
}
