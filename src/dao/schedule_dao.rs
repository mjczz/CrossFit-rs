use crate::entities::schedules::{Model as ScheduleModel};
use super::*;

pub async fn list() -> Result<Vec<ScheduleModel>, DbErr> {
    let db = get_db_ins().await?;
    let list: Vec<ScheduleModel> = Schedules::find().all(&db).await?;
    Ok(list)
}