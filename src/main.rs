mod components;

use components::{
    database::{check_db_connection, connect_to_db},
    server::run_server,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db_ref: Arc<DatabaseConnection> = connect_to_db().await;
    check_db_connection(db_ref).await;

    run_server().await;
}
