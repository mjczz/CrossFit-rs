use sea_orm::{ColumnTrait, QueryFilter, QueryOrder, QuerySelect};
use crate::entities::orders::{Column, Model as OrdersModel};
use super::*;

pub async fn list(shop_id: i32, mut page: u64, page_size: u64) -> Result<Vec<OrdersModel>,
    DbErr> {
    let db = get_db_ins().await?;
    if page <= 0 {
        page = 1
    }
    let offset = (page - 1) * page_size;
    let list: Vec<OrdersModel> = Orders::find().limit(page_size)
        .filter(<crate::entities::orders::Entity as sea_orm::EntityTrait>::Column::ShopId.eq(shop_id))
        .order_by_desc(<crate::entities::orders::Entity as sea_orm::EntityTrait>::Column::StartTime)
        .order_by_desc(<crate::entities::orders::Entity as sea_orm::EntityTrait>::Column::ScheduleId)
        .order_by_desc(<crate::entities::orders::Entity as sea_orm::EntityTrait>::Column::Id)
        .offset(offset)
        .all
        (&db)
        .await?;
    Ok(list)
}