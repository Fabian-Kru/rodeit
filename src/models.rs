use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct User {

    pub id: i64,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub password : String

}

impl User{
    pub fn new() -> User {
        Default::default()
    }

}
#[derive(Serialize, Default, Deserialize, ToSchema)]
pub struct Message {
    pub message: String,
}


impl Message {
    pub fn new() -> Message {
        Default::default()
    }

}


#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct TokenStatus {
    pub token: String,
    pub status: bool,

}

impl TokenStatus {
    pub fn new() -> TokenStatus {
        Default::default()
    }

}


#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

impl crate::LoginRequest {
    pub fn new() -> crate::LoginRequest {
        Default::default()
    }

}

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

impl LoginResponse {
    pub fn new() -> LoginResponse {
        Default::default()
    }

}

