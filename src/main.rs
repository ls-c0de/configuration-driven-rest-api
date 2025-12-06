use std::vec;
use std::future::ready;

use warp::{Filter, filters::BoxedFilter, Reply};

#[tokio::main]
async fn main() {
    //let c: Config = 
    //let conf: Config = load_config();

    //dbg!(conf);

    //let routes = build_warp_routes_from_toml();

    //println!("{:?}", data);

    let paths = vec!["peter", "thommy", "sarah", "john"];
    //let routes = build_filters_from_vec(paths);

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

fn send_answer(stri: String) -> Result<String, ()> {
    if stri.is_empty() == false {
        Ok(format!("Hello from {}", stri))
    } else {
        Err(())
    }
}

fn make_filter(stri: String) -> warp::filters::BoxedFilter<(String, )> {
    warp::path(stri.clone())
        .map(move || format!("Hello from {}", stri))
        .boxed()
}

fn build_filters_from_vec(slice: Vec<&str>) -> BoxedFilter<(String,)> {
    let combined = warp::any()
        .map(|| "".to_string())
        .boxed();

    //let mut temp;
    //for p in slice {
    //    dbg!("{}", p);
    //    temp = combined.or(make_filter(p.to_string())).boxed();
    //}

    combined
}

fn greeting_route(prefix: &'static str)
    -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone
{
    warp::path(prefix)
        .and(warp::path::param::<String>())
        .and(warp::header::<String>("user-agent"))
        .map(move |param: String, agent: String| {
            format!("{} {}, whose agent is {}", prefix, param, agent)
        })
}

fn build_routes()
    -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
    let hi = greeting_route("hello");
    let bye = greeting_route("bye");
    hi.or(bye)
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