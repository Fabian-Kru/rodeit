use std::env;
use std::sync::Arc;

use auth::Claims;
use axum::{
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::{delete, get, post},
};
use axum::body::Body;
use axum::extract::{Path, State};
use axum::routing::patch;

use sqlx::{Error, Pool, Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteQueryResult;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

use crate::models::*;

mod models;

#[derive(OpenApi)]
#[openapi(
info(title = "User API", ),
paths(
get_all_users,
register_user,
login,
delete_user,
get_user_by_id,
patch_user
),
components(schemas(User, LoginResponse, LoginRequest)),
modifiers(& SecuritySchemes),
)]
struct ApiDoc;

struct SecuritySchemes;

impl Modify for SecuritySchemes {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "jwt",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

type Conn = Arc<Pool<Sqlite>>;

#[tokio::main]
async fn main() {
    let database_url = &env::var("DATABASE_URL").unwrap();



    match Sqlite::database_exists(&database_url).await.unwrap() {
        False => {
            sqlx::Sqlite::create_database(&database_url).await.expect("unable to create database");
        }
    }


    let pool = SqlitePool::connect(&database_url).await.unwrap();
    let state_pool = Arc::new(pool);

    sqlx::migrate!("./migrations")
        .run(&*state_pool)
        .await
        .expect("Unable to perform database migration!");

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

/// Get all Users
#[utoipa::path(
get,
path = "/getAllUsers",
responses(
(status = 200, body = [User], description = "operation successful"),
(status = 500, description = "something went wrong")
)
)]
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
            id: Some(x.id),
            name: x.name,
            username: x.username,
            surname: x.surname,
            password: x.password,
        })
    }
    Json(vec)
}

/// Register
///
/// Register a new user
#[utoipa::path(
post,
path = "/register",
responses(
(status = 200, body = User, description = "operation successful"),
(status = 400, description = "user already exists")
)
)]
async fn register_user(State(pool): State<Conn>, Json(user): Json<User>) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        "INSERT INTO user (name, surname, username, password) values ($1,$2,$3,$4)",
    )
        .bind(&user.name)
        .bind(&user.surname)
        .bind(&user.username)
        .bind(&user.password)
        .execute(&(*pool))
        .await;

    if result.is_ok() {
        return Ok(StatusCode::OK);
    }
    return Err(StatusCode::BAD_REQUEST);
}

/// Login
///
/// Login with username and password and return JWT
#[utoipa::path(
post,
path = "/login",
responses(
(status = 200, body = LoginResponse, description = "operation successful"),
(status = 401, description = "user not found and/or password not valid")
)
)]
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

/// Delete a User by ID
#[utoipa::path(
delete,
path = "/user/:user_id",
security(("jwt" = [])),
responses(
(status = 200, description = "operation successful"),
(status = 401, description = "not permitted to delete this user"),
(status = 404, description = "user not found")
)
)]
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
        Err(_) => StatusCode::NOT_FOUND,
    }
}

/// Update a User by ID
#[utoipa::path(
patch,
path = "/user/:user_id",
security(("jwt" = [])),
responses(
(status = 200, description = "operation successful"),
(status = 401, description = "not permitted to change this user"),
(status = 404, description = "user not found")
)
)]
async fn patch_user(
    State(pool): State<Conn>,
    Path(user_id): Path<u32>,
    claims: Claims,
    Json(user_update): Json<UpdateUser>
) -> StatusCode {

    if claims.sub != user_id.to_string() {
        return StatusCode::UNAUTHORIZED;
    }
    let mut res: Option<Result<SqliteQueryResult, Error>> = None;
    if user_update.password != None {
        res = Some(sqlx::query!("UPDATE user set password = $1 WHERE id = $2", user_update.password, user_id)
            .execute(&(*pool))
            .await);
    }
    if user_update.username != None {
        res = Some(sqlx::query!("UPDATE user set username = $1 WHERE id = $2", user_update.username, user_id)
            .execute(&(*pool))
            .await);
    }
    if user_update.name != None {
        res = Some(sqlx::query!("UPDATE user set name = $1 WHERE id = $2", user_update.name, user_id)
            .execute(&(*pool))
            .await);
    }
    if user_update.surname != None {
        res = Some(sqlx::query!("UPDATE user set surname = $1 WHERE id = $2", user_update.surname, user_id)
            .execute(&(*pool))
            .await);
    }

    match res {

        Some(Ok(_)) => StatusCode::OK,
        Some(Err(_)) => StatusCode::NOT_FOUND,
        None => StatusCode::NOT_FOUND
    }
}

/// Get a User by ID
#[utoipa::path(get, path = "/user/:userid",
responses(
(status = 200, description = "operation successful"),
(status = 404, description = "user not found"))
)]
async fn get_user_by_id(State(pool): State<Conn>, Path(user_id): Path<u32>) -> Result<Json<User>, StatusCode> {
    let row = sqlx::query!("SELECT * FROM user WHERE id = $1", user_id)
        .fetch_one(&(*pool))
        .await;

    let Ok(row) = row else {
        return Err(StatusCode::NOT_FOUND);
    };
    return Ok(Json(User {
        id: Some(row.id),
        name: row.name,
        username: row.username,
        surname: row.surname,
        password: row.password,
    }));

}
