// This file will handle the loading process and in the future will incorporate tracker.rs to notify if 
// something in the .yml files changed
use crate::io::design::yaml as yaml;

static ROOT: &str = "configs/api/test.yml";

pub fn load_yml() {
    let input = std::fs::read_to_string(ROOT).unwrap();

    let config: Result<yaml::Yaml, _> = serde_saphyr::from_str(&input);

    match config {
        Ok(parsed_config) => {
            println!("Parsed successfully: {:?}", parsed_config);
        }
        Err(e) => {
            eprintln!("Failed to parse YAML: {}", e);
        }
    }
}