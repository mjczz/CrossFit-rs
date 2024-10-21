use axum::Json;
use http::StatusCode;
use serde::Serialize;
use crate::common_fields;
use crate::entities::courses::Model;

common_fields!(ListRes, ApiData, count);
pub async fn list() -> (StatusCode, Json<ListRes>) {
    match crate::dao::course_dao::list().await{
        Ok(list) => {
            tracing::info!("Database query success: {:?}", list.len());
            let mut data: Vec<ApiData> = vec![];
            for d in list.iter() {
                data.push(d.into())
            }
            (StatusCode::OK, Json(ListRes::new(data)))
        }
        Err(e) => {
            tracing::error!("Database query failed: {:?}", e);
            (StatusCode::BAD_REQUEST, Json(ListRes {
                ..Default::default()
            }))
        }
    }
}

#[derive(Default, Serialize)]
pub struct ApiData {
    pub id: i64,
    pub shop_name: String,
    pub shop_id: i32,
    pub shop_type: i32,
    pub course_date: String,
    pub create_time: Option<i32>,
    pub update_time: Option<i32>,
    pub sync_status: i32,
}

impl From<&Model> for ApiData {
    fn from(d: &Model) -> Self {
        ApiData {
            id: d.id,
            shop_name: d.shop_name.to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            course_date: d.course_date.to_string(),
            create_time: d.create_time,
            update_time: d.update_time,
            sync_status: d.sync_status,
        }
    }
}

impl From<Model> for ApiData {
    fn from(d: Model) -> Self {
        ApiData {
            id: d.id,
            shop_name: d.shop_name.clone().to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            course_date: d.course_date.clone().to_string(),
            create_time: d.create_time,
            update_time: d.update_time,
            sync_status: d.sync_status,
        }
    }
}
