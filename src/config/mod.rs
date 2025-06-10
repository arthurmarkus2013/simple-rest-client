use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{BufReader, BufWriter, Read, Write},
};

use crate::data_types;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub base_url: String,
    pub creds: data_types::Credentials,
}

impl Config {
    pub fn load_config() -> Self {
        let config_file = fs::File::options()
            .create(true)
            .open("./config.json")
            .expect("Failed to open config file");
        let mut reader = BufReader::new(config_file);

        let mut config = String::new();
        reader
            .read_to_string(&mut config)
            .expect("Failed to read config file");

        serde_json::from_str::<Self>(&config).expect("Failed to parse config file")
    }

    pub fn store_config(&self) {
        let config_file = fs::File::options()
            .create(true)
            .open("./config.json")
            .expect("Failed to open config file");
        let mut writer = BufWriter::new(config_file);

        let config = serde_json::to_string(self).expect("Failed to serialize config");

        writer
            .write_all(config.as_bytes())
            .expect("Failed to write config file");
    }
}
