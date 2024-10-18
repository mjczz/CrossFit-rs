use crate::entities::users::{Model as UsersModel};
use super::*;

pub async fn list() -> Result<Vec<UsersModel>, DbErr> {
    let db = get_db_ins().await?;
    let list_users: Vec<UsersModel> = Users::find().all(&db).await?;
    Ok(list_users)
}