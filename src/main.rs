use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let router: Router = Router::new().route("/", get(hello_world));
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let tcp_listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server listening on {}", addr);
    axum::serve(tcp_listener, router).await.unwrap();
}
