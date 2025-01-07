extern crate sea_orm;

use std::sync::Arc;
use sea_orm::{Database, DatabaseConnection, DbErr};
use crate::components::env_vars;

pub async fn connect_to_db() -> Arc<DatabaseConnection> {
    let database_url: String = env_vars::get_env_var("DATABASE_URL");
    let database: DatabaseConnection = Database::connect(database_url).await.unwrap();

    Arc::new(database)
}

pub async fn check_db_connection(db: Arc<DatabaseConnection>) -> () {
    let ping_result: Result<(), DbErr> = db.ping().await;

    match ping_result {
        Ok(_) => println!("💽 Connected to database"),
        Err(e) => println!("💀 Error connecting to database: {}", e),
    }
}
