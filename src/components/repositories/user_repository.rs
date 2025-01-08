use entity::users::{Entity as UserEntity, Model as UserModel};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub async fn get_users(db: DatabaseConnection) -> Result<Vec<UserModel>, DbErr> {
    let users: Vec<UserModel> = UserEntity::find().all(&db).await?;

    Ok(users)
}