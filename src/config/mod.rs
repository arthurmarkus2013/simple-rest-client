use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::data_types::{SessionState, Credentials};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub base_url: String,
    pub creds: Credentials,
}

impl Config {
    pub fn load_config() -> Self {
        let cwd = fs::canonicalize("./").unwrap();

        let config =
            fs::read_to_string(cwd.join(Path::new("config.json"))).unwrap_or("".to_string());

        if !config.is_empty() {
            serde_json::from_str::<Self>(&config).expect("Failed to parse config file")
        } else {
            Self::default()
        }
    }

    pub fn store_config(&self) {
        let cwd = fs::canonicalize("./").unwrap();

        let config = serde_json::to_string(self).expect("Failed to serialize config");

        fs::write(cwd.join(Path::new("config.json")), config).expect("Failed to write config file");
    }

    pub fn current_session_state(&self) -> SessionState {
        if self.creds.current_token.is_empty() {
            SessionState::Unauthenticated
        } else {
            SessionState::Authenticated
        }
    }
}
