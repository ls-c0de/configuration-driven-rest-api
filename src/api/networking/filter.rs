use std::future::ready;
use warp::{Filter};

pub fn build_filter(base: String, paths: Vec<String>) -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::path(base)
        .and(warp::get())
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
        format!("GET on this: {}", path) // Method invokation for db here
    })
}
