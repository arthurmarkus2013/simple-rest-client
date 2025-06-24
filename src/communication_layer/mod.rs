use crate::data_types;

use data_types::Role;

use crate::config;

pub struct DataLayer {
    pub config: config::Config,
    pub movies: Vec<data_types::Movie>,
    client: reqwest::blocking::Client,
}

impl DataLayer {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Simple REST Client/1.0.0")
            .build()
            .expect("Failed to create client");

        Self {
            config: config::Config::load_config(),
            movies: Vec::new(),
            client,
        }
    }

    pub fn register(&self, username: String, password: String, role: Role) -> anyhow::Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow::anyhow!("No base URL provided"));
        }

        if username.is_empty() || password.is_empty() || role == Role::None {
            return Err(anyhow::anyhow!("No credentials provided"));
        }

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
            Role::None => {
                return Err(anyhow::anyhow!("No role provided"));
            }
        }

        let result = self
            .client
            .post(format!("{}/register", self.config.base_url))
            .body(serde_json::to_string(&creds).expect("Failed to serialize user"))
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!("Failed to register user: {}", e)),
        }
    }

    pub fn login(&mut self, username: String, password: String) -> anyhow::Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow::anyhow!("No base URL provided"));
        }

        if username.is_empty() || password.is_empty() {
            return Err(anyhow::anyhow!("No credentials provided"));
        }

        let mut creds = std::collections::HashMap::new();
        creds.insert("username", username);
        creds.insert("password", password);

        let result = self
            .client
            .post(format!("{}/register", self.config.base_url))
            .body(serde_json::to_string(&creds).expect("Failed to serialize user"))
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => {
                self.config.creds = result.json::<data_types::Credentials>()?;

                self.config.store_config();

                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to register user: {}", e)),
        }
    }

    pub fn logout(&mut self) -> anyhow::Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow::anyhow!("No base URL provided"));
        }

        let result = self
            .client
            .post(format!("{}/logout", self.config.base_url))
            .body(serde_json::to_string(&self.config.creds).expect("Failed to serialize token"))
            .send()?;

        match result.error_for_status_ref() {
            Ok(_) => {
                self.config.creds = data_types::Credentials::default();

                self.config.store_config();

                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to logout: {}", e)),
        }
    }

    pub fn create_movie(&self, movie: data_types::Movie) -> anyhow::Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow::anyhow!("No base URL provided"));
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
            Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                Err(anyhow::anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to create movie: {}", e)),
        }
    }

    pub fn list_movies(&self, id: Option<i32>) -> anyhow::Result<Vec<data_types::Movie>> {
        if self.config.base_url.is_empty() {
            return Err(anyhow::anyhow!("No base URL provided"));
        }

        match id {
            Some(id) => {
                if id == -1 {
                    return Ok(Vec::new());
                }

                let result = self
                    .client
                    .post(format!("{}/movie/list/{}", self.config.base_url, id))
                    .header("Authorization", &self.config.creds.current_token)
                    .send()?;

                match result.error_for_status_ref() {
                    Ok(_) => Ok(vec![result.json::<data_types::Movie>()?]),
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
                    .send()?;

                match result.error_for_status_ref() {
                    Ok(_) => Ok(result.json::<Vec<data_types::Movie>>()?),
                    Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                        Err(anyhow::anyhow!("Unauthorized"))
                    }
                    Err(e) => Err(anyhow::anyhow!("Failed to fetch movies: {}", e)),
                }
            }
        }
    }

    pub fn update_movie(&self, movie: data_types::Movie) -> anyhow::Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow::anyhow!("No base URL provided"));
        }

        if movie.id == -1 {
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
            Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                Err(anyhow::anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to update movie: {}", e)),
        }
    }

    pub fn delete_movie(&self, id: i32) -> anyhow::Result<()> {
        if self.config.base_url.is_empty() {
            return Err(anyhow::anyhow!("No base URL provided"));
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
            Err(e) if e.status() == Some(reqwest::StatusCode::UNAUTHORIZED) => {
                Err(anyhow::anyhow!("Unauthorized"))
            }
            Err(e) => Err(anyhow::anyhow!("Failed to delete movie: {}", e)),
        }
    }
}
