use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub current_token: String
}

pub struct Movie {
    pub title: String,
    pub description: String,
    pub release_year: i32
}
