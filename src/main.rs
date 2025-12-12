mod api;
use crate::api::networking::filter::{build_3_step_filter};
use crate::api::networking::server::{start_server_with_route};
use crate::confighandling::structures::yaml::{SimpleLayout, get_test_values};

mod confighandling;
#[allow(unused_imports)]
use crate::confighandling::filehandling::loader::{load_yml, load_config};

#[tokio::main]
async fn main() {
    let layout: SimpleLayout = get_test_values();
    let routes = build_3_step_filter(layout.base, layout.paths);

    start_server_with_route(routes, [127,0,0,1], 3030).await;
}