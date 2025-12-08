use serde::Deserialize;

pub fn load_config() -> Config {

    //check_if_config_exists

    let s = std::fs::read_to_string("configs/config.toml");
    toml::from_str(&s.expect("Error")).unwrap()
}

#[allow(dead_code)]
fn check_if_config_exists() {

}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    api: Api,
    networking: Networking,
    database: Database,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Api {
    name: String,
    description: String,
    version: f32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Networking {
    port: u16, 
    address: String, //convert needed
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Database {
    driver: String,
    connection: String,
}
