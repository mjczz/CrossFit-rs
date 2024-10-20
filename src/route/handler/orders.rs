use axum::Json;
use http::StatusCode;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use crate::dao::{order_dao as dao};
use crate::entities::orders::Model;

pub async fn list_order() -> (StatusCode, Json<OrderRes>) {
    let res = dao::list().await;
    if res.is_ok() {
        let list = res.unwrap();
        println!("user len: {:?}", list.len());
        // this will be converted into a JSON response
        // with a status code of `201 Created`
        let mut data: Vec<OrderData> = vec![];
        for d in list.iter() {
            data.push(d.into())
        }
        return (StatusCode::OK, Json(OrderRes {
            count: data.len(),
            data: data,
        }));
    };

    (StatusCode::BAD_REQUEST, Json(OrderRes {
        ..Default::default()
    }))
}

#[derive(Serialize, Default)]
pub struct OrderRes {
    count: usize,
    data: Vec<OrderData>,
}

#[derive(Default, Serialize)]
pub struct OrderData {
    pub id: i64,
    pub order_id: i32,
    pub schedule_id: i32,
    pub shop_name: String,
    pub shop_id: i32,
    pub shop_type: i32,
    pub status: String,
    pub count: i32,
    pub refund_remarks: String,
    pub total_price: Option<Decimal>,
    pub refund_type: i32,
    pub payment_real_cost: Option<Decimal>,
    pub channel: String,
    pub status_code: i32,
    pub price: Option<Decimal>,
    pub refund_channel: i32,
    pub is_new: Option<i32>,
    pub total_real_price: Option<Decimal>,
    pub is_online: Option<i32>,
    pub remarks: String,
    pub check_in_at: String,
    pub created_at: String,
    pub created_by_username: String,
    pub created_by_phone: String,
    pub username: String,
    pub user_phone: String,
    pub user_id: i32,
    pub user_avatar: String,
    pub user_gender: i32,
    pub card_name: String,
    pub card_balance: Option<Decimal>,
    pub create_time: Option<i32>,
    pub update_time: Option<i32>,
    pub start_ymd_time: String,
    pub start_time: i32,
    pub ymd: i32,
}

impl From<&Model> for OrderData {
    fn from(d: &Model) -> Self {
        OrderData {
            id: d.id,
            order_id: d.order_id,
            schedule_id: d.schedule_id,
            shop_name: d.shop_name.to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            status: d.status.to_string(),
            count: d.count,
            refund_remarks: d.refund_remarks.to_string(),
            total_price: d.total_price,
            refund_type: d.refund_type,
            payment_real_cost: d.payment_real_cost,
            channel: d.channel.to_string(),
            status_code: d.status_code,
            price: d.price,
            refund_channel: d.refund_channel,
            is_new: d.is_new,
            total_real_price: d.total_real_price,
            is_online: d.is_online,
            remarks: d.remarks.to_string(),
            check_in_at: d.check_in_at.to_string(),
            created_at: d.created_at.to_string(),
            created_by_username: d.created_by_username.to_string(),
            created_by_phone: d.created_by_phone.to_string(),
            username: d.username.to_string(),
            user_phone: d.user_phone.to_string(),
            user_id: d.user_id,
            user_avatar: d.user_avatar.to_string(),
            user_gender: d.user_gender,
            card_name: d.card_name.to_string(),
            card_balance: d.card_balance,
            create_time: d.create_time,
            update_time: d.update_time,
            start_ymd_time: d.start_ymd_time.to_string(),
            start_time: d.start_time,
            ymd: d.ymd,
        }
    }
}


impl From<Model> for OrderData {
    fn from(d: Model) -> Self {
        OrderData {
            id: d.id,
            order_id: d.order_id,
            schedule_id: d.schedule_id,
            shop_name: d.shop_name.clone().to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            status: d.status.clone().to_string(),
            count: d.count,
            refund_remarks: d.refund_remarks.clone().to_string(),
            total_price: d.total_price,
            refund_type: d.refund_type,
            payment_real_cost: d.payment_real_cost,
            channel: d.channel.clone().to_string(),
            status_code: d.status_code,
            price: d.price,
            refund_channel: d.refund_channel,
            is_new: d.is_new,
            total_real_price: d.total_real_price,
            is_online: d.is_online,
            remarks: d.remarks.clone().to_string(),
            check_in_at: d.check_in_at.clone().to_string(),
            created_at: d.created_at.clone().to_string(),
            created_by_username: d.created_by_username.clone().to_string(),
            created_by_phone: d.created_by_phone.clone().to_string(),
            username: d.username.clone().to_string(),
            user_phone: d.user_phone.clone().to_string(),
            user_id: d.user_id,
            user_avatar: d.user_avatar.clone().to_string(),
            user_gender: d.user_gender,
            card_name: d.card_name.clone().to_string(),
            card_balance: d.card_balance,
            create_time: d.create_time,
            update_time: d.update_time,
            start_ymd_time: d.start_ymd_time.clone().to_string(),
            start_time: d.start_time,
            ymd: d.ymd,
        }
    }
}


