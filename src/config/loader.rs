// This file will handle the loading process and in the future will incorporate tracker.rs to notify if 
// something in the .yml files changed
use crate::specification::spec::Main as yaml_config;

#[cfg(feature = "loading")]
static YAML_PATH: &str = "settings/api/base.yml";

/// Load the config in api/base.yml
/// 
/// If the config doesn't exist, it will serve test values. 
/// 
/// If the "deserialize" feature is enabled, it will also create deserialized_from_test_values.yml file from test_values().
///  
pub fn load_config() -> yaml_config {
    #[cfg(feature = "loading")]
    {
        crate::config::loader::load_yml()
    }

    #[cfg(not(feature = "loading"))]
    {
        
        #[cfg(feature = "deserialize")]
        {
            deserialize_yaml_into_file();
        }

        yaml_config::test_values()
    }
}

#[cfg(feature = "loading")]
pub fn load_yml() -> yaml_config {
    let input = std::fs::read_to_string(YAML_PATH).unwrap();

    let config: Result<yaml_config, _> = serde_saphyr::from_str(&input);

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

// Deserialize the hardcoded test values into a .yml file for easier editing
#[cfg(feature = "deserialize")]
pub fn deserialize_yaml_into_file() {
    let default = yaml_config::test_values();
    let default_str = serde_saphyr::to_string(&default);

    dbg!("{}", &default_str);

    let _ = std::fs::write("settings/api/deserialized_from_test_values.yml", default_str.unwrap()); // FIXME: Check needed, if something goes wrong!
}