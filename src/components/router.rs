use crate::components::{database, handlers::{health_check, user_router}};
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn get_db() -> DatabaseConnection {
    let db: DatabaseConnection = database::connect_to_db().await;

    db
}

pub async fn get_router() -> Router {
    let health_check_handler = get(health_check::handler);
    let user_router = user_router::get_user_router();
    
    let db: DatabaseConnection = get_db().await;
    let app_state = AppState { db };

    Router::new()
        .route("/", health_check_handler)
        .nest("/users", user_router)
        .with_state(app_state)
}
