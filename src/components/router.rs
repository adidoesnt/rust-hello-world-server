use super::database::connect_to_db;
use crate::components::handlers::health_check;
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub async fn get_db_ref() -> Arc<DatabaseConnection> {
    let db_ref: Arc<DatabaseConnection> = connect_to_db().await;

    db_ref.clone()
}

pub async fn get_router() -> Router {
    let health_check_handler = get(health_check::handler);
    let db_ref: Arc<DatabaseConnection> = get_db_ref().await;

    Router::new()
        .route("/", health_check_handler)
        .with_state(db_ref)
}
