use axum::{routing::get, Router};
use crate::components::{controllers::user_controller, router::AppState};

pub fn get_user_router() -> Router<AppState> {
    let get_all_users_handler = get(user_controller::get_all_users);

    Router::new().route("/", get_all_users_handler)
}