use sea_orm::{DatabaseConnection, ConnectionTrait, Database, EntityTrait, QueryTrait, DbErr}; // 确保导入 EntityTrait 和 QueryTrait
pub mod user_dao;
pub mod teacher_dao;
pub mod order_dao;
pub mod course_dao;
pub mod schedule_dao;
use crate::entities::prelude::{*};

const DATABASE_URL: &str = "sqlite:///Users/ccc/p2159.sqllite";

pub async fn get_db_ins() -> Result<DatabaseConnection, DbErr> {
    Database::connect(DATABASE_URL).await
}