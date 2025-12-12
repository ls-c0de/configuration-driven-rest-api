mod api;
use crate::api::networking::server::{start_server, start_server_with_base_values_locally};

mod confighandling;
use crate::confighandling::filehandling::loader::{load_yml, load_config};

#[tokio::main]
async fn main() {
    start_server_with_base_values_locally().await;
}