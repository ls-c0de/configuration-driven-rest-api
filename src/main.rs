mod api;
mod confighandling;

#[cfg(feature = "http")]
use crate::api::networking::filter;
#[cfg(feature = "http")]
use crate::api::networking::filter::{build_3_step_filter};
#[cfg(feature = "http")]
use crate::api::networking::server::{start_server_with_route};
#[cfg(feature = "http")]
use crate::confighandling::structures::yaml::{SimpleLayout, get_test_values};
#[cfg(feature = "loading")]
use crate::confighandling::filehandling::loader::{load_yml, load_config};

#[tokio::main]
async fn main() {
     #[cfg(feature = "database")]
    println!("Test");


    #[cfg(feature = "http")]
    http().await;
}

#[cfg(feature = "http")]
async fn http() {
    let layout: SimpleLayout = get_test_values();
    let routes = build_3_step_filter(layout.base, layout.paths);

    start_server_with_route(routes, [127,0,0,1], 3030).await;
}