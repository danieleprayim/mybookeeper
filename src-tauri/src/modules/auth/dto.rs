use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String
}