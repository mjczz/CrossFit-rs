//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::task::Poll::{Pending, Ready};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
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
    #[sea_orm(column_type = "custom(\"numeric\")", nullable)]
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
