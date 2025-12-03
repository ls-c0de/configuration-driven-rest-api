use warp::{Filter, Reply};
use serde::Deserialize;


#[tokio::main]
async fn main() {
    //let c: Config = 
    //let conf: Config = load_config();

    //dbg!(conf);

    //let routes = build_warp_routes_from_toml();

    //println!("{:?}", data);

    let routes = build_routes();

    serve(routes).await
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

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Config {
    greeting: Vec<Greeting>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Greeting {
    name: String,
    text: String,
}

#[allow(dead_code)]
fn load_config() -> Config {
    let s = std::fs::read_to_string("configs/easy.toml");
    toml::from_str(&s.expect("Error")).unwrap()
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