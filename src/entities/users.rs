//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_id: i32,
    pub shop_name: String,
    pub shop_id: i32,
    pub shop_type: i32,
    pub gender: i32,
    pub username: String,
    pub phone: String,
    pub avatar: String,
    pub create_time: Option<i32>,
    pub update_time: Option<i32>,
    pub last_course_time: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
