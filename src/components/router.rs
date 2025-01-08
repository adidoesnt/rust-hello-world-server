use axum::{routing::get, Router};
use crate::components::handlers::health_check;

pub async fn get_router() -> Router {
    let health_check_handler = get(health_check::handler);

    Router::new().route("/", health_check_handler)
}