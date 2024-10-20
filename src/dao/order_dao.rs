use sea_orm::{QueryOrder, QuerySelect};
use crate::entities::orders::{Model as OrdersModel};
use super::*;

pub async fn list() -> Result<Vec<OrdersModel>, DbErr> {
    let db = get_db_ins().await?;
    let list: Vec<OrdersModel> = Orders::find().limit(10).all(&db)
        .await?;
    Ok(list)
}