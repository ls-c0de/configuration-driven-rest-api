mod api;
mod confighandling;
mod db;

#[cfg(feature = "http")]
use crate::api::http;

#[cfg(feature = "database")]
use crate::db::*;

#[tokio::main]
async fn main() {
    #[cfg(feature = "database")]
    db().await;

    #[cfg(feature = "http")]
    http().await;
}