// Here we will define the yml structure in structs to use in our loader.rs
// Maybe we can combine the loading process to get a more streamlined error handling -> Done
use serde::{ Deserialize, Serialize};

// Kinda unnecessary but whatever for now
pub fn get_test_values() -> SimpleLayout {
    SimpleLayout {
        base: "api".to_string(),
        paths: vec![
        "foo".to_string(),
        "bar".to_string(),
        "foo/bar".to_string(),
        "foo/bar/foo".to_string(),
        ],
    }
}

// Kinda unnecessary but whatever for now
pub struct SimpleLayout { 
    pub base: String,
    pub paths: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Yaml {
    pub api: Api,
    pub endpoints: Vec<Endpoints>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    pub name: String,
    pub version: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoints {
    pub path: String,
    pub methods: Vec<String>,
    pub description: String,
}

impl Default for Yaml {
    fn default() -> Self {
        Yaml {
            api: Api::default(),
            endpoints: vec![Endpoints::default(), Endpoints::set_costum_path("path2")],
        }
    }
}

impl Default for Api {
    fn default() -> Self {
        Api {
            name: "base".to_string(),
            version: 1.0,
        }
    }
}

impl Default for Endpoints {
    fn default() -> Self {
        Endpoints {
            path: "path1".to_string(),
            methods: vec!["GET".to_string(), "PUT".to_string()],
            description: "This is a test Description".to_string(),
        }
    }
}

impl Endpoints {
    fn set_costum_path(name: &str) -> Self {
        Endpoints {
            path: name.to_string(),
            methods: vec!["GET".to_string(), "PUT".to_string()],
            description: "This is a test Description".to_string(),
        }
    }
}