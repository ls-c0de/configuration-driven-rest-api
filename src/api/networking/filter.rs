use std::future::ready;
use warp::reply::Json;
use warp::{Filter};
use warp::http;

pub fn build_3_step_filter(base: String, paths: Vec<String>) -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Clone {
    warp::path(base)
        .and(warp::path::tail())
        .and(warp::method())
        .and_then(move |tail: warp::path::Tail, method: http::Method| {
           request_handler(paths.clone(), tail, method)
        })
    // .map(|reply: warp::reply::Json| {
    //     response_handler(reply)
    // })
}

fn request_handler(paths: Vec<String>, tail: warp::path::Tail, method: http::Method) -> impl warp::Future<Output = Result<warp::reply::Json, warp::Rejection>> {
    let req = Dummy {
                path: tail.as_str().to_string(),
                method: method.clone(),
            };

            if paths.contains(&req.path) {
                ready(Ok(response_handler(req)))
                //ready(Ok(warp::reply::json(&req)))
            } else {
                ready(Err(warp::reject::not_found()))
            }
}

fn response_handler(req: Dummy) -> warp::reply::Json {
    let result = Response {
        data: get_data_from_db(req.path),
        method: req.method.to_string(),
    };

    warp::reply::json(&result)
    //format!("{} on this: {}", req.method, req.path) // Method invokation for db here
}

fn get_data_from_db(path: String) -> String {
    format!("Data for path: {}", path)
}

#[derive(serde::Serialize)]
struct Response {
    data: String,
    method: String,
}

struct Dummy {
    path: String,
    method: http::Method,
}


