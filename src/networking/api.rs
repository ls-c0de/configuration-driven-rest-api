use std::vec;
use std::future::ready;
use warp::{Filter, Reply};

fn build_filter(base: String, paths: Vec<String>) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::path(base)
        .and(warp::path::tail())
        .and_then(move |tail: warp::path::Tail| {
            let path = tail.as_str().to_string();

            if paths.contains(&path) {
                ready(Ok(path))
            } else {
                ready(Err(warp::reject::not_found()))
            }
        })
    .map(|path: String| {
        format!("You are here: {}", path) // Method invokation for db here
    })
}

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

    let routes = build_filter(base, paths);

    serve(routes, address, port).await;
}

/// Starts the server with default base and paths for quick testing on port 3030.
///
/// ## Default Values
/// * Base: "api"
/// * Paths: ["hello", "bye", "hello/bye"]
/// 
pub async fn start_server_with_base_values_locally() {
    let base = "api".to_string();
    let paths = vec![
        "hello".to_string(),
        "bye".to_string(),
        "hello/bye".to_string(),
    ];

    start_server(base, paths, [127,0,0,1], 3030).await;
}