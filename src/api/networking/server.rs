use crate::confighandling::structures::yaml::{SimpleLayout, get_test_values};
use warp::{Filter, Reply};
use warp::Rejection;
use crate::api::networking::filter::build_3_step_filter;

async fn serve<F>(routes: F, address: [u8; 4], port: u16)
where
    F: Filter + Clone + Send + Sync + 'static,
    F::Extract: Reply,
{
    warp::serve(routes)
        .run((address, port))
        .await;
}

/// Starts the server with the given route filter.
/// 
pub async fn start_server_with_route<T: Filter<Extract = (warp::reply::Json,), Error = Rejection> + Clone + Send + Sync + 'static>
(routes: T, address: [u8; 4], port: u16) {
    // let paths: Vec<String> = paths
    //     .into_iter()
    //     .map(String::from)
    //     .collect();

    serve(routes, address, port).await;
}

/// Starts the server with default base and paths for quick testing on port 3030.
///
#[allow(dead_code)] 
pub async fn start_server_localhost(values: Option<SimpleLayout>) {
    match values {
        Some(v) => {
            let route = build_3_step_filter(v.base, v.paths);
            start_server_with_route(route, [127,0,0,1], 3030).await;
            return;
        },
        None => {
            let v: SimpleLayout = get_test_values();
            let route = build_3_step_filter(v.base, v.paths);
            start_server_with_route(route, [127,0,0,1], 3030).await;
        }
    }
}