use serde::{Deserialize, Serialize};
use toml::Table;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host_ip: String,
    pub user_id: String, 
}

pub fn config_path_exists() -> bool {
    false
}