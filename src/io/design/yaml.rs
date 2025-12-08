// Here we will define the yml structure in structs to use in our loader.rs
// Maybe we can combine the loading process to get a more streamlined error handling
use serde::{ Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Yaml {
    api: Api,
    endpoints: Endpoints,
}

#[derive(Serialize, Deserialize, Debug)]
struct Api {
    name: String,
    version: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Endpoints {
    path: String,
    method: String,
    description: String,
}