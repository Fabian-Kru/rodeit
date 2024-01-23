use std::collections::BTreeMap;
use std::sync::Arc;
use std::{env, process};

use axum::extract::{Path, State};
use axum::routing::patch;
use axum::{
	http::StatusCode,
	response::IntoResponse,
	routing::{delete, get, post},
	Json, Router, Server,
};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::SignWithKey;
use sha2::Sha256;
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::*;

mod models;

#[derive(OpenApi)]
#[openapi(paths(get_all_users, register_user, login, delete_user, get_user_by_id, patch_user),
components(schemas(User, LoginResponse, LoginRequest)),
tags((name = "user-service", description = "Test")))]
struct ApiDoc;

type Conn = Arc<Pool<Sqlite>>;

#[tokio::main]
async fn main() {
	load_config();
	println!("{}", &env::var("DATABASE_URL").unwrap());
	let url = "sqlite://test.db";
	let pool = SqlitePool::connect(&url).await.unwrap();

	let state_pool = Arc::new(pool);

	sqlx::migrate!("./migrations")
		.run(&*state_pool)
		.await
		.expect("Error!!!");

	let app = Router::new()
		.route("/getAllUsers", get(get_all_users))
		.route("/register", post(register_user))
		.route("/login", post(login))
		.route("/user/:user_id", get(get_user_by_id))
		.route("/user/:user_id", delete(delete_user))
		.route("/user/:user_id", patch(patch_user))
		.merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
		.with_state(state_pool);

	Server::bind(&"127.0.0.1:9001".parse().unwrap())
		.serve(app.into_make_service())
		.await
		.unwrap();
}
#[utoipa::path(get, path = "/getAllUsers", responses((status = 200, body = [User]), (status = 404)))]
async fn get_all_users(State(pool): State<Conn>) -> Json<Vec<User>> {
	let row = sqlx::query!("SELECT * FROM user")
		.fetch_all(&(*pool))
		.await
		.unwrap();

	if row.is_empty() {
		return Json(Vec::new());
	}
	let mut vec = Vec::new();

	for x in row {
		vec.push(User {
			id: x.id,
			name: x.name,
			username: x.username,
			surname: x.surname,
			password: x.password,
		})
	}
	Json(vec)
}

#[utoipa::path(post, path = "/register", responses(
(status = 200, body = User), (status = 404)))]
async fn register_user(State(pool): State<Conn>, Json(user): Json<User>) -> impl IntoResponse {
	let result = sqlx::query(
		"INSERT INTO user (name, surname, username, password, id) values ($1,$2,$3,$4,$5)",
	)
	.bind(&user.name)
	.bind(&user.surname)
	.bind(&user.username)
	.bind(&user.password)
	.bind(&user.id)
	.execute(&(*pool))
	.await;

	if result.is_ok() {
		return (
			StatusCode::OK,
			Json(Message {
				message: "yeay".to_string(),
			}),
		);
	}
	return (
		StatusCode::BAD_REQUEST,
		Json(Message {
			message: "".to_string(),
		}),
	);
}

#[utoipa::path(post, path = "/verify", responses(
(status = 200, body = TokenStatus), (status = 404)))]
async fn verify_token(Json(token): Json<TokenStatus>) -> Json<TokenStatus> {
	let t = token.token;
	println!("Verify {t}");
	Json(TokenStatus {
		token: "Testuser".to_string(),
		status: false,
	})
}

#[utoipa::path(post, path = "/login", responses(
(status = 200, body = LoginResponse), (status = 400)))]
async fn login(
	State(pool): State<Conn>,
	Json(login): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
	let token = "None".to_string();
	let username = login.username;
	let password = login.password;
	println!("Login-Request {username} {password}");

	if username == "admin" && password == "admin" {
		let mut claims = BTreeMap::new();
		// payload data
		claims.insert("user", "admin");
		// secret-key
		let key: Hmac<Sha256> = Hmac::new_from_slice(token.as_ref()).unwrap();
		let token_r: String = claims.sign_with_key(&key).unwrap();
		println!("Token {}", token_r);

		return Ok(Json(LoginResponse { token: token_r }));
	} else {
		let row = sqlx::query!(
			"SELECT * FROM user WHERE username = $1 and password = $2",
			username,
			password
		)
		.fetch_one(&(*pool))
		.await;

		if row.ok().is_none() {
			return Err(StatusCode::UNAUTHORIZED);
		}
		let mut claims = BTreeMap::new();
		// payload data
		claims.insert("user", "admin");
		// secret-key
		let key: Hmac<Sha256> = Hmac::new_from_slice(token.as_ref()).unwrap();
		let token_r: String = claims.sign_with_key(&key).unwrap();
		return Ok(Json(LoginResponse { token: token_r }));
	}
	Err(StatusCode::UNAUTHORIZED)
}
fn load_config() {
	if !env::var("SECRET_TOKEN").is_ok() {
		println!("SECRET_TOKEN not found");
		process::exit(1)
	}
}

#[utoipa::path(delete, path = "/user/:user_id", responses(
(status = 200), (status = 404)))]
async fn delete_user(State(pool): State<Conn>, Path(user_id): Path<u32>) -> StatusCode {
	sqlx::query!("DELETE FROM user WHERE id = $1", user_id).execute(&(*pool));
	StatusCode::OK
}

#[utoipa::path(patch, path = "/user/:user_id/", responses(
(status = 200), (status = 404)))]
async fn patch_user(State(pool): State<Conn>, Path(user_id): Path<u32>) -> StatusCode {
	StatusCode::OK
}

#[utoipa::path(get, path = "/user/:userid", responses(
(status = 200), (status = 404)))]
async fn get_user_by_id(State(pool): State<Conn>, Path(user_id): Path<u32>) -> Json<User> {
	let row = sqlx::query!("SELECT * FROM user WHERE id = $1", user_id)
		.fetch_one(&(*pool))
		.await
		.unwrap();

	return Json(User {
		id: row.id,
		name: row.name,
		username: row.username,
		surname: row.surname,
		password: row.password,
	});
}
