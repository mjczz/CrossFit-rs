use sea_orm::QuerySelect;
use crate::entities::courses::{Model as CourseModel};
use super::*;

pub async fn list() -> Result<Vec<CourseModel>, DbErr> {
    let db = get_db_ins().await?;
    let list: Vec<CourseModel> = Courses::find().limit(20).all(&db).await?;
    Ok(list)
}