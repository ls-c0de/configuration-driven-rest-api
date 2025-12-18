use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    name: String, //
    settings: Settings, // Settings
    model: Vec<Model>, // DB logic
    endpoint: Vec<Endpoint> // Endpoint logic
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    tablename: String,
    fields: Vec<Fields<()>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fields<T> {
    name: String,
    datatype: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    description: String, // description
    path: String, // path under to serve
    method: String, // REST method
    table: String, // table where to serve
}