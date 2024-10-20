use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::dao::{user_dao};
use crate::entities::users::Model;

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}


pub async fn list_user() -> (StatusCode, Json<UsersRes>) {
    let res = user_dao::list().await;
    if res.is_ok() {
        let list = res.unwrap();
        println!("user len: {:?}", list.len());
        // this will be converted into a JSON response
        // with a status code of `201 Created`
        let mut data: Vec<UserData> = vec![];
        for d in list.iter() {
            // 使用 From trait 进行转换
            // let user_data: UserData = UserData::from(d.clone());
            data.push(d.into())
        }
        return (StatusCode::OK, Json(UsersRes {
            count: data.len(),
            data: data,
        }));
    };

    (StatusCode::BAD_REQUEST, Json(UsersRes {
        ..Default::default()
    }))
}

#[derive(Serialize, Default)]
pub struct UsersRes {
    count: usize,
    data: Vec<UserData>,
}

#[derive(Default, Serialize)]
pub struct UserData {
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

// 从 &Model（引用）转换为 UserData：
// 这个实现可以避免不必要的拷贝，适用于不需要转移所有权的场景。
// 引用版本（From<&Model>）：适用于你想保留 Model 的场景，因为它只借用了 Model 的数据，而不会消耗所有权。这种方式避免了不必要的内存分配和数据拷贝，尤其在需要遍历或只读场景下更高效。
// 如果你有一个 &Model，可以使用引用实现：
// let user_data: UserData = (&model).into();
impl From<&Model> for UserData {
    fn from(d: &Model) -> Self {
        UserData {
            id: d.id,
            user_id: d.user_id,
            shop_name: d.shop_name.clone(),
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            gender: d.gender,
            username: d.username.clone(),
            phone: d.phone.clone(),
            avatar: d.avatar.clone(),
            create_time: d.create_time,
            update_time: d.update_time,
            last_course_time: d.last_course_time,
        }
    }
}

// 从 Model（所有权）转换为 UserData：
// 这个实现直接消耗 Model 的所有权，适用于你不再需要 Model 的场景。
// 所有权版本（From<Model>）：适用于你不再需要 Model，希望消耗它来创建 UserData。这种方式可以避免再复制数据，比如调用 clone()。
// 如果你有一个 Model 实例，想直接转移所有权，可以使用所有权实现：
// let user_data: UserData = model.into();
impl From<Model> for UserData {
    fn from(d: Model) -> Self {
        UserData {
            id: d.id,
            user_id: d.user_id,
            shop_name: d.shop_name,  // 这里不再需要 clone，直接使用 d 的所有权
            shop_id: d.shop_id,
            shop_type: d.shop_type,
            gender: d.gender,
            username: d.username,
            phone: d.phone,
            avatar: d.avatar,
            create_time: d.create_time,
            update_time: d.update_time,
            last_course_time: d.last_course_time,
        }
    }
}

