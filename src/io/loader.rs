fn load_config() -> Config {
    let s = std::fs::read_to_string("configs/easy.toml");
    toml::from_str(&s.expect("Error")).unwrap()
}

#[derive(Debug, Deserialize)]
struct Config {
    greeting: Vec<Greeting>,
}

#[derive(Debug, Deserialize)]
struct Greeting {
    name: String,
    text: String,
}