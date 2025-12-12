// This file will handle the loading process and in the future will incorporate tracker.rs to notify if 
// something in the .yml files changed
#[cfg(feature = "loading")]
use crate::confighandling::structures::yaml as yaml_config;
#[cfg(feature = "loading")]
use crate::confighandling::structures::toml as toml_config;

#[cfg(feature = "loading")]
static YAML_PATH: &str = "settings/api/test.yml";
#[cfg(feature = "loading")]
static BASE_TOML: &str = "settings/config.toml";

#[cfg(feature = "loading")]
pub fn load_yml() -> yaml_config::Yaml {
    let input = std::fs::read_to_string(YAML_PATH).unwrap();

    let config: Result<yaml_config::Yaml, _> = serde_saphyr::from_str(&input);

    return config.unwrap()

// FIXME: Handling needed!
//    match config {
//        Ok(parsed_config) => {
//            println!("Parsed successfully: {:?}", parsed_config);
//        }
//        Err(e) => {
//            eprintln!("Failed to parse YAML: {}", e);
//        }
//    }
}

/// Load the config.
/// 
/// If the config doesn't exists, it will generate a new one.
/// If it fails to get the status of the config it will serve default values,
/// but will not generate a new one.
/// 
#[cfg(feature = "loading")]
pub fn load_config() -> toml_config::Config {
    let check = std::fs::exists(BASE_TOML);

    match check {
        Ok(true) =>  {
            // read file normally

            let s = std::fs::read_to_string(BASE_TOML);
            toml::from_str(&s.expect("Error")).unwrap() // FIXME: Better Error Handling needed
        },
        Ok(false) => {
            // create file with default values

            let default = toml_config::get_default_config();
            let default_str = toml::to_string_pretty(&default).expect("Something went wrong!");
            let _ = std::fs::write(BASE_TOML, default_str); // FIXME: Check needed, if something goes wrong!

            return default
        },
        Err(_) => {
            // return hardcoded values
            println!("Something went wrong!");
            println!("Giving default values");
            println!("Skipping generating file...");
            println!("Have you checked your Permissions?");

            toml_config::get_default_config()
        }
    }
}

#[cfg(feature = "loading")]
pub fn deserialize_yaml_into_file() {
    let default = yaml_config::Yaml::default();
    let default_str = serde_saphyr::to_string(&default);

    dbg!("{}", &default_str);

    let _ = std::fs::write(YAML_PATH, default_str.unwrap()); // FIXME: Check needed, if something goes wrong!
}