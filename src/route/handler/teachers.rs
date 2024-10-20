use axum::Json;
use http::StatusCode;
use serde::Serialize;
use crate::dao::teacher_dao;
use crate::entities::teachers::Model;

#[derive(Serialize, Default)]
pub struct TeachersRes {
    count: usize,
    data: Vec<TeacherData>,
}

pub async fn list_teacher() -> (StatusCode, Json<TeachersRes>) {
    let res = teacher_dao::list().await;
    if res.is_ok() {
        let list = res.unwrap();
        println!("user len: {:?}", list.len());
        // this will be converted into a JSON response
        // with a status code of `201 Created`
        let mut data: Vec<TeacherData> = vec![];
        for d in list.iter() {
            // 使用 From trait 进行转换
            // let user_data: UserData = UserData::from(d.clone());
            data.push(d.into())
        }
        return (StatusCode::OK, Json(TeachersRes {
            count: data.len(),
            data: data,
        }));
    };

    (StatusCode::BAD_REQUEST, Json(TeachersRes {
        ..Default::default()
    }))
}

#[derive(Default, Serialize)]
pub struct TeacherData {
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

impl From<&Model> for TeacherData {
    fn from(d: &Model) -> Self {
        TeacherData {
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


