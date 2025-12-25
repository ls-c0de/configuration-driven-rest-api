mod api;
mod config;
mod db;
mod specification;

#[cfg(feature = "http")]
use crate::api::http;

#[cfg(feature = "database")]
use crate::db::*;

#[cfg(feature = "deserialize")]
use crate::config::loader::deserialize_yaml_into_file;

#[tokio::main]
async fn main() {
    #[cfg(feature = "database")]
    db().await;

    #[cfg(feature = "http")]
    http().await;

    #[cfg(feature = "deserialize")]
    deserialize_yaml_into_file();
}