#[allow(dead_code)]
fn load_config() -> Config {
    let s = std::fs::read_to_string("configs/easy.toml");
    toml::from_str(&s.expect("Error")).unwrap()
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Config {
    greeting: Vec<Greeting>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Greeting {
    name: String,
    text: String,
}