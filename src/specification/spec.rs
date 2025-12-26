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
            model: vec![Model::test_values("users".into()), Model::test_values("posts".into())],
            endpoint: vec![Endpoint::test_values()],
        }
    }
}


///
/// Settings for the API server
/// Not used right now
/// 
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

///
/// Model representing a database table
/// 
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
    pub fn test_values(name: String) -> Self {
        Self {
            tablename: name,
            fields: vec![
                Fields::test_values("id".to_string(), "INTEGER".to_string(), Some("PRIMARY KEY NOT NULL".to_string())),
                Fields::test_values("email".to_string(), "INTEGER".to_string(), None),
            ],
        }
    }
}

///
/// Field representing a column in a database table
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct Fields<T> {
    pub name: String,
    pub datatype: T,
    pub constraints: Option<String>,
}

impl<T: Default> Default for Fields<T> {
    fn default() -> Self {
        Self {
            name: String::new(),
            datatype: T::default(),
            constraints: None,
        }
    }
}

impl Fields<String> {
    fn test_values(fieldn: String, dt: String, constraints: Option<String>) -> Self {
        Self {
            name: fieldn,
            datatype: dt.to_string(),
            constraints: constraints,
        }
    }
}

///
/// Endpoint representing an API endpoint
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    pub description: String,
    pub path: String,
    pub method: String,
    pub table: String,
    pub op: String,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            description: String::new(),
            path: String::new(),
            method: String::new(),
            table: String::new(),
            op: String::new(),
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
            op: "get_by_id".into(),
        }
    }
}
