//mod networking;
//use crate::networking::api::{start_server_with_base_values_locally};

mod io;
use crate::io::setup::{load_config};

#[tokio::main]
async fn main() {
    let conf = load_config();
    dbg!("{}", conf);

    //start_server_with_base_values_locally().await;
}