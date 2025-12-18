use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub name: String,
    pub settings: Settings,
    pub model: Vec<Model>,
    pub endpoint: Vec<Endpoint>,
}

impl Default for Main {
    fn default() -> Self {
        Self {
            name: String::new(),
            settings: Settings::default(),
            model: Vec::new(),
            endpoint: Vec::new(),
        }
    }
}

impl Main {
    pub fn test_values() -> Self {
        Self {
            name: "example-api".into(),
            settings: Settings::test_values(),
            model: vec![Model::test_values()],
            endpoint: vec![Endpoint::test_values()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {

}

impl Default for Settings {
    fn default() -> Self {
        Self {}
    }
}

impl Settings {
    pub fn test_values() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    pub tablename: String,
    pub fields: Vec<Fields<String>>, // FIXME: Only String right now 
}

impl Default for Model {
    fn default() -> Self {
        Self {
            tablename: String::new(),
            fields: Vec::new(),
        }
    }
}

impl Model {
    pub fn test_values() -> Self {
        Self {
            tablename: "users".into(),
            fields: vec![
                Fields::test_values("FIELD 1".to_string()),
                Fields::test_values("FIELD 2".to_string()),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fields<T> {
    pub name: String,
    pub datatype: T,
}

impl<T: Default> Default for Fields<T> {
    fn default() -> Self {
        Self {
            name: String::new(),
            datatype: T::default(),
        }
    }
}

impl Fields<String> {
    fn test_values(text: String) -> Self {
        Self {
            name: String::new(),
            datatype: text.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    pub description: String,
    pub path: String,
    pub method: String,
    pub table: String,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            description: String::new(),
            path: String::new(),
            method: String::new(),
            table: String::new(),
        }
    }
}

impl Endpoint {
    pub fn test_values() -> Self {
        Self {
            description: "List users".into(),
            path: "/users".into(),
            method: "GET".into(),
            table: "users".into(),
        }
    }
}
