use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub current_token: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub release_year: i32,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum Role {
    #[default]
    None,
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum SessionState {
    #[default]
    Unauthenticated,
    Authenticated,
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role {
            _ if role == "Admin" => Role::Admin,
            _ if role == "User" => Role::User,
            _ => Role::Admin,
        }
    }
}
