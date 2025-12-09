use serde::{ Deserialize, Serialize};

static ROOT: &str = "configs/config.toml";

/// Load the config.
/// 
/// If the config doesn't exists, it will generate a new one.
/// If it fails to get the status of the config it will serve default values,
/// but will not generate a new one.
/// 
#[allow(dead_code)]
pub fn load_config() -> Config {
    let check = std::fs::exists(ROOT);

    match check {
        Ok(true) =>  {
            // read file normally

            let s = std::fs::read_to_string(ROOT);
            toml::from_str(&s.expect("Error")).unwrap() // FIXME: Better Error Handling needed
        },
        Ok(false) => {
            // create file with default values

            let default = get_default_config();
            let default_str = toml::to_string_pretty(&default).expect("Something went wrong!");
            let _ = std::fs::write(ROOT, default_str); // FIXME: Check needed, if something goes wrong!

            return default
        },
        Err(_) => {
            // return hardcoded values
            println!("Something went wrong!");
            println!("Giving default values");
            println!("Skipping generating file...");
            println!("Have you checked your Permissions?");

            get_default_config()
        }
    }
}

fn get_default_config() -> Config {
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

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api: Api,
    pub networking: Networking,
    pub database: Database,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Api {
    pub name: String,
    pub description: String,
    pub version: f32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Networking {
    pub port: u16, 
    pub address: String, //convert needed
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub driver: String,
    pub connection: String,
}
