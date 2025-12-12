use std::future::ready;
use warp::{Filter};
use warp::http;

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

struct Dummy {
    path: String,
    method: http::Method,
}
