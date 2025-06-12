use crate::data_types;

use data_types::Role;
use serde::{Serialize, ser};

use crate::config;

pub struct DataLayer {
    config: config::Config,
    movies: Vec<data_types::Movie>,
    client: reqwest::Client,
}

impl DataLayer {
    pub fn new() -> Self {
        let config = config::Config::load_config();

        let client = reqwest::Client::builder()
            .user_agent("Simple REST Client/1.0.0")
            .build()
            .expect("Failed to create client");

        Self {
            config,
            movies: Vec::new(),
            client,
        }
    }

    pub async fn register(
        &self,
        username: String,
        password: String,
        role: Role,
    ) -> anyhow::Result<()> {
        let mut creds = std::collections::HashMap::new();
        creds.insert("username", username);
        creds.insert("password", password);

        match role {
            Role::User => {
                creds.insert("role", "user".into());
            }
            Role::Admin => {
                creds.insert("role", "admin".into());
            }
        }

        let result = self
            .client
            .post(format!("{}/register", self.config.base_url))
            .body(serde_json::to_string(&creds).expect("Failed to serialize user"))
            .send()
            .await
            .expect("Failed to register user");

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to register user: {}", e)),
        }
    }

    pub async fn login(&mut self, username: String, password: String) -> anyhow::Result<()> {
        let mut creds = std::collections::HashMap::new();
        creds.insert("username", username);
        creds.insert("password", password);

        let result = self
            .client
            .post(format!("{}/register", self.config.base_url))
            .body(serde_json::to_string(&creds).expect("Failed to serialize user"))
            .send()
            .await
            .expect("Failed to register user");

        match result.error_for_status_ref() {
            Ok(_) => {
                self.config.creds = result.json::<data_types::Credentials>().await?;

                self.config.store_config();

                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to register user: {}", e)),
        }
    }

    pub async fn logout(&mut self) -> anyhow::Result<()> {
        let result = self
            .client
            .post(format!("{}/logout", self.config.base_url))
            .body(serde_json::to_string(&self.config.creds).expect("Failed to serialize token"))
            .send()
            .await
            .expect("Failed to logout");

        match result.error_for_status_ref() {
            Ok(_) => {
                self.config.creds = data_types::Credentials::default();

                self.config.store_config();

                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to logout: {}", e)),
        }
    }

    pub async fn create_movie(&self, movie: data_types::Movie) -> anyhow::Result<()> {
        let result = self
            .client
            .post(format!("{}/movie/create", self.config.base_url))
            .body(serde_json::to_string(&movie).expect("Failed to serialize movie"))
            .header("Authorization", &self.config.creds.current_token)
            .send()
            .await
            .expect("Failed to create movie");

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                Err(anyhow::anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to create movie: {}", e)),
        }
    }

    pub async fn list_movies(&self, id: Option<i32>) -> anyhow::Result<Vec<data_types::Movie>> {
        match id {
            Some(id) => {
                let result = self
                    .client
                    .post(format!("{}/movie/list/{}", self.config.base_url, id))
                    .header("Authorization", &self.config.creds.current_token)
                    .send()
                    .await
                    .expect("Failed to fetch movie");

                match result.error_for_status_ref() {
                    Ok(_) => Ok(vec![result.json::<data_types::Movie>().await?]),
                    Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                        Err(anyhow::anyhow!("Unauthorized"))
                    }
                    Err(e) => Err(anyhow::anyhow!("Failed to fetch movie: {}", e)),
                }
            }

            None => {
                let result = self
                    .client
                    .post(format!("{}/movie/list", self.config.base_url))
                    .header("Authorization", &self.config.creds.current_token)
                    .send()
                    .await
                    .expect("Failed to fetch movies");

                match result.error_for_status_ref() {
                    Ok(_) => Ok(result.json::<Vec<data_types::Movie>>().await?),
                    Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                        Err(anyhow::anyhow!("Unauthorized"))
                    }
                    Err(e) => Err(anyhow::anyhow!("Failed to fetch movies: {}", e)),
                }
            }
        }
    }

    pub async fn update_movie(&self, movie: data_types::Movie) -> anyhow::Result<()> {
        let result = self
            .client
            .post(format!(
                "{}/movie/update/{}",
                self.config.base_url, movie.id
            ))
            .body(serde_json::to_string(&movie).expect("Failed to serialize movie"))
            .header("Authorization", &self.config.creds.current_token)
            .send()
            .await
            .expect("Failed to update movie");

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                Err(anyhow::anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to update movie: {}", e)),
        }
    }

    pub async fn delete_movie(&self, id: i32) -> anyhow::Result<()> {
        let result = self
            .client
            .delete(format!("{}/movie/delete/{}", self.config.base_url, id))
            .header("Authorization", &self.config.creds.current_token)
            .send()
            .await
            .expect("Failed to delete movie");

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                Err(anyhow::anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to delete movie: {}", e)),
        }
    }
}
