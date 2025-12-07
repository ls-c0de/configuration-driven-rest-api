use std::vec;
use std::future::ready;
use warp::{Filter,  Reply};

fn build_filter(base: String, paths: Vec<String>) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::path(base)
        .and(
            warp::path::param()
                .and_then(move |path: String| {
                    if paths.contains(&path.to_string()) {
                        ready(Ok(path))
                    } else {
                        ready(Err(warp::reject::not_found()))
                    }
                })
        )
        .map(|path: String| {
            format!("You are here: {}", path)
        })
}

async fn serve<F>(routes: F)
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    warp::serve(routes)
        .run(([127,0,0,1], 3030))
        .await;
}

/// Starts the server with predefined paths for testing purposes.
/// 
/// # Arguments
/// * `base` - A string slice that holds the base path for the server.
/// * `paths` - A vector of strings representing valid paths under the base.
/// 
pub async fn start_server(base: String, paths: Vec<String>) {
    let paths = paths
        .into_iter()
        .map(String::from)
        .collect();

    let routes = build_filter(base, paths);

    serve(routes).await;
}

/// Starts the server with default base and paths for quick testing.
///
pub async fn start_server_with_base_values() {
    let base = "api".to_string();
    let paths = vec![
        "users".to_string(),
        "posts".to_string(),
        "comments".to_string(),
    ];

    start_server(base, paths).await;
}