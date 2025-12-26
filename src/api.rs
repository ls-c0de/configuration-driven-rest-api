#[cfg(feature = "http")]
pub mod filter;
#[cfg(feature = "http")]
pub mod methods;
#[cfg(feature = "http")]
pub mod server;

#[cfg(feature = "http")]
use crate::api::filter::{build_3_step_filter};

#[cfg(feature = "http")]
use crate::api::server::{start_server_with_route};

#[cfg(feature = "http")]
// Entry point for REST-Api stuff
pub async fn http() {
    use crate::specification::spec::Main;
    use crate::config::loader::load_config;

    println!("Starting Server");
    let config: Main = load_config();
    let routes = build_3_step_filter(config.name, config.endpoint.iter().map(|e| e.path.clone()).collect());

    start_server_with_route(routes, [127,0,0,1], 3030).await;
}