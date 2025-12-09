mod networking;
use crate::networking::api::{start_server};

mod io;
use crate::io::setup::{load_config};
use crate::io::design::loader::{load_yml};

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