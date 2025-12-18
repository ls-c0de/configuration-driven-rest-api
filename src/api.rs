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
use crate::specification::legacy::{SimpleLayout, get_test_values};

#[cfg(feature = "http")]
// Entry point for REST-Api stuff
pub async fn http() {
    println!("Starting Server");
    let layout: SimpleLayout = get_test_values();
    let routes = build_3_step_filter(layout.base, layout.paths);

    start_server_with_route(routes, [127,0,0,1], 3030).await;
}