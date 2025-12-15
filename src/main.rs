mod api;
mod confighandling;

#[cfg(feature = "loading")]
use crate::confighandling::filehandling::loader::{load_yml, load_config};

#[cfg(feature = "http")]
use crate::api::http;

#[tokio::main]
async fn main() {
     #[cfg(feature = "database")]
    println!("Test");


    #[cfg(feature = "http")]
    http().await;
}