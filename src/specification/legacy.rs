// legacy methods, working on new ones

// Kinda unnecessary but whatever
#[cfg(feature = "http")]
pub fn get_test_values() -> SimpleLayout {
    SimpleLayout {
        base: "api".to_string(),
        paths: vec![
        "foo".to_string(),
        "bar".to_string(),
        "foo/bar".to_string(),
        "foo/bar/foo".to_string(),
        ],
    }
}

// Kinda unnecessary but whatever
#[cfg(feature = "http")]
pub struct SimpleLayout { 
    pub base: String,
    pub paths: Vec<String>,
}