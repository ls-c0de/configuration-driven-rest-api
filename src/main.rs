use std::vec;

use warp::{Filter, filters::BoxedFilter, Reply};

#[tokio::main]
async fn main() {
    //let c: Config = 
    //let conf: Config = load_config();

    //dbg!(conf);

    //let routes = build_warp_routes_from_toml();

    //println!("{:?}", data);

    let paths = vec!["hallo", "bye"];
    let routes = build_filters_from_vec(paths);

    serve(routes);
}

fn make_filter(stri: &str) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    warp::path(stri)
        .map(move || format!("Hello from {}", path))
        .boxed()
}



fn build_filters_from_vec(slice: Vec<&str>) 
    -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone
{
    let iterator = slice.iter();
    
    let mut filters = warp::any()
        .map(|| "".to_string())
        .boxed();

    // build paths and combine warp filters into one, then return it
    for element in iterator {
        dbg!("{}", element);

        let path = element.clone();

        let filter = warp::path(path)
            .map(move || format!("Hi, you accessed {}", path))
            .boxed();

        filters = filters.or(filter).untuple_one().boxed();
    }

    // Dynamisch Filter generieren und kombinieren
    let mut combined = warp::any().map(|| "Nothing").boxed();
    for p in slice {
        combined = combined.or(make_filter(p)).boxed();
    }

    return filters
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