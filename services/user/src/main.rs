use std::collections::BTreeMap;
use std::sync::Arc;
use std::{env, process};

use auth::Claims;
use axum::extract::{Path, State};
use axum::routing::patch;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

#[utoipa::path(post, path = "/login", responses(
(status = 200, body = LoginResponse), (status = 400)))]
async fn login(
    State(pool): State<Conn>,
    Json(login): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let username = login.username;
    let password = login.password;
    println!("Login-Request {username} {password}");

    let row = sqlx::query!(
        "SELECT * FROM user WHERE username = $1 and password = $2",
        username,
        password
    )
    .fetch_one(&(*pool))
    .await;

    let Ok(row) = row else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let claims = Claims::new(row.id.to_string());
    let token = claims.encode().map_err(|err| err.0)?;

    return Ok(Json(LoginResponse { token }));
}

#[utoipa::path(delete, path = "/user/:user_id", responses(
(status = 200), (status = 401), (status = 404)))]
async fn delete_user(
    State(pool): State<Conn>,
    Path(user_id): Path<u32>,
    claims: Claims,
) -> StatusCode {
    if claims.sub != user_id.to_string() {
        return StatusCode::UNAUTHORIZED;
    }

    let res = sqlx::query!("DELETE FROM user WHERE id = $1", user_id)
        .execute(&(*pool))
        .await;

    match res {
        Ok(_) => StatusCode::OK,
        // TODO: is this semantically correct?
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[utoipa::path(patch, path = "/user/:user_id/", responses(
(status = 200), (status = 401), (status = 404)))]
async fn patch_user(
    State(pool): State<Conn>,
    Path(user_id): Path<u32>,
    claims: Claims,
) -> StatusCode {
    if claims.sub != user_id.to_string() {
        return StatusCode::UNAUTHORIZED;
    }

    // TODO: implement
    return StatusCode::NOT_IMPLEMENTED;
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
