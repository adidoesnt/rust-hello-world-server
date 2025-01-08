mod components;

use components::server::run_server;

#[tokio::main]
async fn main() {
    run_server().await;
}
