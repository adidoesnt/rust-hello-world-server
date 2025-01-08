use crate::components::{router::AppState, services::user_service};
use axum::{extract::State, Json};
use entity::users::Model as UserModel;

pub async fn get_all_users(State(state): State<AppState>) -> Json<Vec<UserModel>> {
    let users: Vec<UserModel> = user_service::get_all_users(state.db).await.unwrap();

    Json(users)
}
