mod components;

use axum::{routing::get, Router};
use components::handlers::hello_world;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use components::{database::{connect_to_db, check_db_connection}, env_vars};
use sea_orm::DatabaseConnection;

#[tokio::main]
async fn main() {
    let db_ref: Arc<DatabaseConnection> = connect_to_db().await;
    check_db_connection(db_ref).await;

    let port: String = env_vars::get_env_var("PORT");
    let parsed_port: u16 = port.parse::<u16>().unwrap();

    let router: Router = Router::new().route("/", get(hello_world::handler));
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], parsed_port));
    let tcp_listener: TcpListener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server listening on {}", addr);
    axum::serve(tcp_listener, router).await.unwrap();
}
