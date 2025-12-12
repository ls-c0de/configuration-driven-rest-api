use std::future::ready;
use warp::{Filter};
use warp::http;

/// deprecated: use build_3_step_filter instead
/// 
pub fn build_filter(base: String, paths: Vec<String>) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::path(base)
        .and(warp::path::tail())
        .and(warp::method())
        .and_then(move |tail: warp::path::Tail, method: http::Method| {
            let req = Dummy {
                path: tail.as_str().to_string(),
                method: method.clone(),
            };

            if paths.contains(&req.path) {
                ready(Ok(req))
            } else {
                ready(Err(warp::reject::not_found()))
            }
        })
    .map(|req: Dummy| {
        format!("{} on this: {}", req.method, req.path) // Method invokation for db here
    })
}

pub fn build_3_step_filter(base: String, paths: Vec<String>) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::path(base)
        .and(warp::path::tail())
        .and(warp::method())
        .and_then(move |tail: warp::path::Tail, method: http::Method| {
           request_handler(paths.clone(), tail, method)
        })
    .map(|req: Dummy| {
        response_handler(req)
    })
}

fn request_handler(paths: Vec<String>, tail: warp::path::Tail, method: http::Method) -> impl warp::Future<Output = Result<Dummy, warp::Rejection>> {
    let req = Dummy {
                path: tail.as_str().to_string(),
                method: method.clone(),
            };

            if paths.contains(&req.path) {
                ready(Ok(req))
            } else {
                ready(Err(warp::reject::not_found()))
            }
}

fn response_handler(req: Dummy) -> String {
    format!("{} on this: {}", req.method, req.path) // Method invokation for db here
}

struct Dummy {
    path: String,
    method: http::Method,
}
