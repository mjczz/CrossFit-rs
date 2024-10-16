mod entities;

use sea_orm::{Database, DbErr, EntityTrait, QueryTrait}; // 确保导入 EntityTrait 和 QueryTrait
const DATABASE_URL: &str = "sqlite:///Users/ccc/p2159.sqllite";
use crate::entities::users::{Entity as UsersEntity, Model as UsersModel}; // Entity 和 Model 导入
use crate::entities::teachers::{Entity as TeacherEntity, Model as TeacherModel}; // Entity 和
use crate::entities::prelude::{*};
// Model 导入

// 使用 tokio::main 来启动异步运行时
#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // 连接数据库
    let db = Database::connect(DATABASE_URL).await?;

    // 查询所有 users 表中的记录
    let _list_users: Vec<UsersModel> = Users::find().all(&db).await?;
    // for user in list_users.iter() {
    //     println!("{:?\n\n}", user);
    //     println!("");
    // }

    // 查询所有 teachers 表中的记录
    let list_teachers: Vec<TeacherModel> = Teachers::find().all(&db).await?;
    for teacher in list_teachers.iter() {
        println!("{:?\n\n}", teacher);
        println!("");
    }

    Ok(())
}