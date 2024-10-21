use axum::Json;
use http::StatusCode;
use sea_orm::prelude::Decimal;
use serde::Serialize;
use crate::common_fields;
use crate::entities::schedules::Model;

common_fields!(ListRes, ApiData, count);
pub async fn list() -> (StatusCode, Json<ListRes>) {
    match crate::dao::schedule_dao::list().await{
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
    pub course_id: i32,
    pub schedule_id: i32,
    pub shop_name: String,
    pub shop_id: i32,
    pub shop_type: i32,
    pub max_users: i32,
    pub start: String,
    pub end: String,
    pub ymd: i32,
    pub course_name: String,
    pub week_day: String,
    pub description: String,
    pub difficulty_level: i32,
    pub course_length: i32,
    pub course_type_tag: String,
    pub teacher_username: String,
    pub teacher_gender: i32,
    pub teacher_score: Option<Decimal>,
    pub teacher_id: i32,
    pub teacher_avatar: String,
    pub create_time: Option<i32>,
    pub update_time: Option<i32>,
    pub start_fmt: String,
    pub end_fmt: String,
}

impl From<&Model> for ApiData {
    fn from(d: &Model) -> Self {
        ApiData {
            id: d.id,
            course_id: d.course_id,
            schedule_id: d.schedule_id,
            shop_name: d.shop_name.clone().to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            max_users: d.max_users,
            start: d.start.clone().to_string(),
            end: d.end.clone().to_string(),
            ymd: d.ymd,
            course_name: d.course_name.clone().to_string(),
            week_day: d.week_day.clone().to_string(),
            description: d.description.clone().to_string(),
            difficulty_level: d.difficulty_level,
            course_length: d.course_length,
            course_type_tag: d.course_type_tag.clone().to_string(),
            teacher_username: d.teacher_username.clone().to_string(),
            teacher_gender: d.teacher_gender,
            teacher_score: d.teacher_score,
            teacher_id: d.teacher_id,
            teacher_avatar: d.teacher_avatar.clone().to_string(),
            create_time: d.create_time,
            update_time: d.update_time,
            start_fmt: d.start_fmt.clone().to_string(),
            end_fmt: d.end_fmt.clone().to_string(),
        }
    }
}

impl From<Model> for ApiData {
    fn from(d: Model) -> Self {
        ApiData {
            id: d.id,
            course_id: d.course_id,
            schedule_id: d.schedule_id,
            shop_name: d.shop_name.to_string(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            max_users: d.max_users,
            start: d.start.to_string(),
            end: d.end.to_string(),
            ymd: d.ymd,
            course_name: d.course_name.to_string(),
            week_day: d.week_day.to_string(),
            description: d.description.to_string(),
            difficulty_level: d.difficulty_level,
            course_length: d.course_length,
            course_type_tag: d.course_type_tag.to_string(),
            teacher_username: d.teacher_username.to_string(),
            teacher_gender: d.teacher_gender,
            teacher_score: d.teacher_score,
            teacher_id: d.teacher_id,
            teacher_avatar: d.teacher_avatar.to_string(),
            create_time: d.create_time,
            update_time: d.update_time,
            start_fmt: d.start_fmt.to_string(),
            end_fmt: d.end_fmt.to_string(),
        }
    }
}
