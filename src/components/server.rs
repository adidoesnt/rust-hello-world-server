use super::{env_vars, router};
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn get_tcp_listener() -> TcpListener {
    let port: String = env_vars::get_env_var("PORT");
    let parsed_port: u16 = port.parse::<u16>().unwrap();

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], parsed_port));

    TcpListener::bind(addr).await.unwrap()
}

pub async fn run_server() {
    let router: Router = router::get_router().await;

    let tcp_listener: TcpListener = get_tcp_listener().await;
    let addr: SocketAddr = tcp_listener.local_addr().unwrap();

    println!("🚀 Server listening on {}", addr);
    axum::serve(tcp_listener, router).await.unwrap();
}
