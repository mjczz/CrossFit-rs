use sea_orm::QuerySelect;
use crate::entities::courses::{Model as CourseModel};
use super::*;

pub async fn list(shop_id: i32) -> Result<Vec<CourseModel>, DbErr> {
    let db = get_db_ins().await?;
    let list: Vec<CourseModel> = Courses::find().
        limit(20).
        filter(<crate::entities::courses::Entity as sea_orm::EntityTrait>::Column::ShopId.eq(shop_id)).
        all(&db).await?;
    Ok(list)
}