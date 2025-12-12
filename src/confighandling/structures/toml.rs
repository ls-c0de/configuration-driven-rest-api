#[cfg(feature = "loading")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "loading")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api: Api,
    pub networking: Networking,
    pub database: Database,
}

#[cfg(feature = "loading")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub name: String,
    pub description: String,
    pub version: f32,
}

#[cfg(feature = "loading")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Networking {
    pub port: u16, 
    pub address: String, //convert needed
}

#[cfg(feature = "loading")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub driver: String,
    pub connection: String,
}

#[cfg(feature = "loading")]
pub fn get_default_config() -> Config {
    return Config {
        api: Api {
            name: "Default".to_string(),
            description: "Default description".to_string(),
            version: 1.0,
        },
        networking: Networking {
            port: 8080,
            address: "127.0.0.1".to_string(),
        },
        database: Database {
            driver: "".to_string(),
            connection: "".to_string(),
        },
    };
}