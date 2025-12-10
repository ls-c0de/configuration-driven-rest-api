mod api;
use crate::api::networking::filter::{start_server};

mod confighandling;
use crate::confighandling::filehandling::loader::{load_yml, load_config};

#[tokio::main]
async fn main() {
    let conf = load_config();
    let address = conf.networking.address;
    let port = conf.networking.port;

    let design = load_yml();
    let base = design.api.name;

    let paths: Vec<String> = design.endpoints.as_slice()
        .into_iter()
        .map(|e| e.path.clone())
        .collect();

    dbg!(address);
    dbg!(port);

    start_server(base, paths, [127,0,0,1], 3030).await
}