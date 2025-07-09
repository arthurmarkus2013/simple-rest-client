use std::{collections::HashMap, io::Read};

use crate::data_types;

use data_types::Role;

use crate::config::Config;
use crate::data_types::Movie;
use reqwest::{StatusCode, blocking::Client};

use anyhow::{Result, anyhow};

pub struct DataLayer {
    pub config: Config,
    pub movies: Vec<Movie>,
    client: Client,
}

impl DataLayer {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Simple REST Client/1.0.0")
            .build()
            .expect("Failed to create client");

        Self {
            config: Config::load_config(),
            movies: Vec::new(),
            client,
        }
    }

    pub fn register(&self, username: String, password: String, role: Role) -> Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow!("No base URL provided"));
        }

        if username.is_empty() || password.is_empty() || role == Role::None {
            return Err(anyhow!("No credentials provided"));
        }

        let mut creds = HashMap::new();
        creds.insert("username", username);
        creds.insert("password", password);

        match role {
            Role::User => {
                creds.insert("role", "user".into());
            }
            Role::Admin => {
                creds.insert("role", "admin".into());
            }
            Role::None => {
                return Err(anyhow!("No role provided"));
            }
        }

        let result = self
            .client
            .post(format!("{}/register", self.config.base_url))
            .body(serde_json::to_string(&creds).expect("Failed to serialize user"))
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("Failed to register user: {}", e)),
        }
    }

    pub fn login(&mut self, username: String, password: String) -> Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow!("No base URL provided"));
        }

        if username.is_empty() || password.is_empty() {
            return Err(anyhow!("No credentials provided"));
        }

        self.config.creds.username = username.clone();
        self.config.creds.password = password.clone();

        let mut creds = HashMap::new();
        creds.insert("username", username);
        creds.insert("password", password);

        let result = self
            .client
            .post(format!("{}/login", self.config.base_url))
            .body(serde_json::to_string(&creds).expect("Failed to serialize user"))
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => {
                let token = result.json::<HashMap<String, String>>()?;

                self.config.creds.current_token = token["token"].clone();

                self.config.store_config();

                Ok(())
            }
            Err(e) => Err(anyhow!("Failed to log in: {}", e)),
        }
    }

    pub fn logout(&mut self) -> Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow!("No base URL provided"));
        }

        let result = self
            .client
            .post(format!("{}/logout", self.config.base_url))
            .header("Authorization", &self.config.creds.current_token)
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => {
                self.config.creds.current_token.clear();

                self.config.store_config();

                Ok(())
            }
            Err(e) => Err(anyhow!("Failed to log out: {}", e)),
        }
    }

    pub fn create_movie(&mut self, movie: Movie) -> Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow!("No base URL provided"));
        }

        if movie.id == -1 {
            return Ok(());
        }

        let result = self
            .client
            .post(format!("{}/movie/create", self.config.base_url))
            .body(serde_json::to_string(&movie).expect("Failed to serialize movie"))
            .header("Authorization", &self.config.creds.current_token)
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) if e.status() == Some(StatusCode::UNAUTHORIZED) => {
                self.config.creds.current_token.clear();
                self.config.store_config();

                Err(anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow!("Failed to create movie: {}", e)),
        }
    }

    pub fn list_movies(&mut self, id: Option<i32>) -> Result<Vec<Movie>> {
        if self.config.base_url.is_empty() {
            return Err(anyhow!("No base URL provided"));
        }

        match id {
            Some(id) => {
                if id == -1 {
                    return Ok(Vec::new());
                }

                let mut result = self
                    .client
                    .get(format!("{}/movie/list/{}", self.config.base_url, id))
                    .header("Authorization", &self.config.creds.current_token)
                    .send()?;

                match result.error_for_status_ref() {
                    Ok(_) => {
                        let mut body = String::new();
                        let _ = result.read_to_string(&mut body)?;
                        let data: HashMap<String, Movie> = match serde_json::from_str(&body) {
                            Ok(data) => data,
                            Err(_) => HashMap::<String, Movie>::new(),
                        };

                        if data.is_empty() {
                            self.movies = vec![data["movie"].clone()];

                            Ok(self.movies.clone())
                        } else {
                            self.movies.clear();

                            Err(anyhow!("No movie found"))
                        }
                    }
                    Err(e) if e.status() == Some(StatusCode::UNAUTHORIZED) => {
                        self.config.creds.current_token.clear();
                        self.config.store_config();

                        Err(anyhow!("Unauthorized"))
                    }
                    Err(e) => Err(anyhow!("Failed to fetch movie: {}", e)),
                }
            }

            None => {
                let mut result = self
                    .client
                    .get(format!("{}/movie/list", self.config.base_url))
                    .header("Authorization", &self.config.creds.current_token)
                    .send()?;

                match result.error_for_status_ref() {
                    Ok(_) => {
                        let mut body = String::new();
                        let _ = result.read_to_string(&mut body)?;
                        let data: HashMap<String, Vec<Movie>> = match serde_json::from_str(&body) {
                            Ok(data) => data,
                            Err(_) => HashMap::<String, Vec<Movie>>::new(),
                        };

                        if !data.is_empty() {
                            self.movies = data["movies"].clone();

                            Ok(self.movies.clone())
                        } else {
                            self.movies.clear();

                            Err(anyhow!("No movies found"))
                        }
                    }
                    Err(e) if e.status() == Some(StatusCode::UNAUTHORIZED) => {
                        self.config.creds.current_token.clear();
                        self.config.store_config();

                        Err(anyhow!("Unauthorized"))
                    }
                    Err(e) => Err(anyhow!("Failed to fetch movies: {}", e)),
                }
            }
        }
    }

    pub fn update_movie(&mut self, movie: Movie) -> Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow!("No base URL provided"));
        }

        if movie.id == -1 || movie == Movie::default() {
            return Ok(());
        }

        let result = self
            .client
            .post(format!(
                "{}/movie/update/{}",
                self.config.base_url, movie.id
            ))
            .body(serde_json::to_string(&movie).expect("Failed to serialize movie"))
            .header("Authorization", &self.config.creds.current_token)
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) if e.status() == Some(StatusCode::UNAUTHORIZED) => {
                self.config.creds.current_token.clear();
                self.config.store_config();

                Err(anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow!("Failed to update movie: {}", e)),
        }
    }

    pub fn delete_movie(&mut self, id: i32) -> Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow!("No base URL provided"));
        }

        if id == -1 {
            return Ok(());
        }

        let result = self
            .client
            .delete(format!("{}/movie/delete/{}", self.config.base_url, id))
            .header("Authorization", &self.config.creds.current_token)
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) if e.status() == Some(StatusCode::UNAUTHORIZED) => {
                self.config.creds.current_token.clear();
                self.config.store_config();

                Err(anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow!("Failed to delete movie: {}", e)),
        }
    }
}
