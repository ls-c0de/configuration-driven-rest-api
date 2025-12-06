use std::vec;
use std::future::ready;
use warp::{Filter,  Reply};

#[tokio::main]
async fn main() {
    let paths = vec!["peter", "thommy", "sarah", "john"];

    let routes = warp::path("paths")
        .and(
            warp::path::param()
                .and_then(move |path: String| {
                    if paths.contains(&path.as_str()) {
                        ready(Ok(path))
                    } else {
                        ready(Err(warp::reject::not_found()))
                    }
                })
                //.untuple_one(),
        )
        .map(|path: String| {
            format!("You are here: {}", path)
        });

    serve(routes).await;
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