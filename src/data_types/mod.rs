use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub current_token: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub release_year: i32,
}

#[derive(Serialize, Deserialize, Default)]
pub enum Role {
    #[default]
    Admin,
    User,
}
