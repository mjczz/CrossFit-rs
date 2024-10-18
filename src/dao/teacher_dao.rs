use crate::entities::teachers::{Model as TeacherModel};
use super::*;

pub async fn list() -> Result<Vec<TeacherModel>, DbErr> {
    let db = get_db_ins().await?;
    let list_teacher: Vec<TeacherModel> = Teachers::find().all(&db).await?;
    Ok(list_teacher)
}