use std::vec;
use warp::{Filter, Reply};
use crate::api::networking::filter;

async fn serve<F>(routes: F, address: [u8; 4], port: u16)
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    warp::serve(routes)
        .run((address, port))
        .await;
}

/// Starts the server with the given base and paths.
/// 
/// ## Arguments
/// * `base` - A string slice that holds the base path for the server.
/// * `paths` - A vector of strings representing valid paths under the base.
/// * `port` - The port number on which the server will listen.
/// 
pub async fn start_server(base: String, paths: Vec<String>, address: [u8; 4], port: u16) {
    let paths = paths
        .into_iter()
        .map(String::from)
        .collect();

    let routes = filter::build_filter(base, paths);

    serve(routes, address, port).await;
}

/// Starts the server with default base and paths for quick testing on port 3030.
///
/// ## Default Values
/// * Base: "api"
/// * Paths: ["hello", "bye", "hello/bye"]
///
#[allow(dead_code)] 
pub async fn start_server_with_base_values_locally() {
    let base = "api".to_string();
    let paths = vec![
        "foo".to_string(),
        "bar".to_string(),
        "foo/bar".to_string(),
        "foo/bar/foo".to_string(),
    ];

    start_server(base, paths, [127,0,0,1], 3030).await;
}