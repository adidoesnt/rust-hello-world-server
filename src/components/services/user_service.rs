use crate::components::repositories::user_repository;
use entity::users::Model as UserModel;
use sea_orm::{DatabaseConnection, DbErr};

pub async fn get_all_users(db: DatabaseConnection) -> Result<Vec<UserModel>, DbErr> {
    let users: Vec<UserModel> = user_repository::get_users(db).await?;

    Ok(users)
}