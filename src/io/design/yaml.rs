// Here we will define the yml structure in structs to use in our loader.rs
// Maybe we can combine the loading process to get a more streamlined error handling
use serde::{ Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Yaml {
    api: Api,
    endpoints: Endpoints,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    name: String,
    version: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoints {
    path: String,
    method: Vec<String>,
    description: String,
}

impl Default for Yaml {
    fn default() -> Self {
        Yaml {
            api: Api::default(),
            endpoints: Endpoints::default(),
        }
    }
}

impl Default for Api {
    fn default() -> Self {
        Api {
            name: "test".to_string(),
            version: 1.0,
        }
    }
}

impl Default for Endpoints {
    fn default() -> Self {
        Endpoints {
            path: "test".to_string(),
            method: vec!["GET".to_string()],
            description: "This is a test Description".to_string(),
        }
    }
}