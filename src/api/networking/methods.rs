#[cfg(feature = "http")]
pub fn get_request(path: String) -> Result<warp::reply::Json, warp::Rejection> {
    Ok(warp::reply::json(&format!("GET request received for path: {}", path)))
}

#[cfg(feature = "http")]
pub fn post_request(path: String) -> Result<warp::reply::Json, warp::Rejection>  {
    Ok(warp::reply::json(&format!("POST request received for path: {}", path)))
}

#[cfg(feature = "http")]
pub fn put_request(path: String) -> Result<warp::reply::Json, warp::Rejection>  {    
    Ok(warp::reply::json(&format!("PUT request received for path: {}", path)))
}