use axum::Json;
use http::StatusCode;
use serde::Serialize;
use crate::common_fields;
use crate::dao::teacher_dao;
use crate::entities::teachers::Model;

common_fields!(ListRes, ApiData, count);
pub async fn list() -> (StatusCode, Json<ListRes>) {
    let res = teacher_dao::list().await;
    if res.is_ok() {
        let list = res.unwrap();
        tracing::info!("Database query success: {:?}", list.len());
        let mut data: Vec<ApiData> = vec![];
        for d in list.iter() {
            data.push(d.into())
        }
        return (StatusCode::OK, Json(ListRes::new(data)));
    };
    (StatusCode::BAD_REQUEST, Json(ListRes {
        ..Default::default()
    }))
}

#[derive(Default, Serialize)]
pub struct ApiData {
    pub id: i64,
    pub teacher_id: i32,
    pub shop_name: String,
    pub shop_id: i32,
    pub shop_type: i32,
    pub gender: i32,
    pub name: String,
    pub phone: String,
    pub avatar: String,
    pub create_time: Option<i32>,
    pub update_time: Option<i32>,
}

impl From<&Model> for ApiData {
    fn from(d: &Model) -> Self {
        ApiData {
            id: d.id,
            teacher_id: d.teacher_id,
            shop_name: d.shop_name.clone().to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            gender: d.gender,
            name: d.name.clone().to_string(),
            phone: d.phone.clone().to_string(),
            avatar: d.avatar.clone().to_string(),
            create_time: d.create_time,
            update_time: d.update_time,
        }
    }
}

impl From<Model> for ApiData {
    fn from(d: Model) -> Self {
        ApiData {
            id: d.id,
            teacher_id: d.teacher_id,
            shop_name: d.shop_name.to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            gender: d.gender,
            name: d.name.to_string(),
            phone: d.phone.to_string(),
            avatar: d.avatar.to_string(),
            create_time: d.create_time,
            update_time: d.update_time,
        }
    }
}


