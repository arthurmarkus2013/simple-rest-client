use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub current_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub release_year: i32,
}

#[derive(Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}
