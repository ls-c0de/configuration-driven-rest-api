//mod networking;
//use crate::networking::api::{start_server_with_base_values_locally};

mod io;
use crate::io::setup::{load_config};
use crate::io::design::loader::{deserialize_yaml_into_file, load_yml};

#[tokio::main]
async fn main() {
    let conf = load_config();
    dbg!("{}", conf);

    //load_yml();

    deserialize_yaml_into_file();
    //start_server_with_base_values_locally().await;
}