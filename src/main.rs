mod env_vars;
mod handlers;

use axum::{routing::get, Router};
use handlers::hello_world;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port: String = env_vars::get_env_var("PORT");
    let parsed_port: u16 = port.parse::<u16>().unwrap();

    let router: Router = Router::new().route("/", get(hello_world::handler));
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], parsed_port));
    let tcp_listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server listening on {}", addr);
    axum::serve(tcp_listener, router).await.unwrap();
}
