use std::future::ready;
use warp::reply::Json;
use warp::{Filter};
use warp::http;
use crate::api::networking::methods;

pub fn build_3_step_filter(base: String, paths: Vec<String>) -> impl Filter<Extract = (Json,), Error = warp::Rejection> + Clone {
    warp::path(base)
        .and(warp::path::tail())
        .and(warp::method())
        .and_then(move |tail: warp::path::Tail, method: http::Method| {
           request_handler(paths.clone(), tail, method)
        })
}

fn request_handler(paths: Vec<String>, tail: warp::path::Tail, method: http::Method) -> impl warp::Future<Output = Result<warp::reply::Json, warp::Rejection>> {
    let req = Dummy {
                path: tail.as_str().to_string(),
                method: method.clone(),
            };

            match response_handler(req, paths) {
                Ok(json) => ready(Ok(json)),
                Err(_) => ready(Err(warp::reject::not_found())),
            }
}
// Should we even use 2 separate functions for request and response handling?
fn response_handler(req: Dummy, paths: Vec<String>) -> Result<warp::reply::Json, warp::Rejection> {
    if paths.contains(&req.path) {
                match req.method {
                    http::Method::GET => methods::get_request(req.path),
                    http::Method::POST => methods::post_request(req.path),
                    http::Method::PUT => methods::put_request(req.path),
                    _ => Err(warp::reject::not_found()),
                }
            } else {
                Err(warp::reject::not_found())
            }
    // let result = Response {
    //     data: get_data_from_db(req.path),
    //     method: req.method.to_string(),
    // };
}

// Code found on StackOverflow to handle rejections globally
// We should probably use something like it in the future 
//
// fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
//     if err.is_not_found() {
//         Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
//     } else if let Some(e) = err.find::<AppErr>() {
//         Ok(reply::with_status(e.reason.as_str(), StatusCode::BAD_REQUEST))
//     } else {
//         eprintln!("unhandled rejection: {:?}", err);
//         Ok(reply::with_status(
//             "INTERNAL_SERVER_ERROR",
//             StatusCode::INTERNAL_SERVER_ERROR,
//         ))
//     }
// }

#[derive(serde::Serialize)]
struct Response {
    data: String,
    method: String,
}

struct Dummy {
    path: String,
    method: http::Method,
}


