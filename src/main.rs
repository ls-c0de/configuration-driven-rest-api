mod networking;
use crate::networking::api::{start_server_with_base_values_locally};

#[tokio::main]
async fn main() {
    start_server_with_base_values_locally().await;
}