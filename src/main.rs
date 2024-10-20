mod entities;
mod dao;

use sea_orm::{DbErr};
use crate::dao::{*};
use crate::entities::{*};
use std::{thread, time};
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::entities::users::UserData;

async fn say_world() {
    let ten_millis = time::Duration::from_secs(2);
    thread::sleep(ten_millis);
    println!("world");
}

// 使用 tokio::main 来启动异步运行时
#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let op = say_world();

    let list_teacher: Vec<teachers::Model> = teacher_dao::list().await?;
    for teacher in list_teacher.iter() {
        println!("{:?} : {:?}", teacher.name, teacher.teacher_id);
        println!("");
    }
    println!("teacher len: {:?}", list_teacher.len());


    let list_order: Vec<orders::Model> = order_dao::list().await?;
    println!("order len: {:?}", list_order.len());

    let list_course: Vec<courses::Model> = course_dao::list().await?;
    println!("course len: {:?}", list_course.len());

    let list_schedule: Vec<schedules::Model> = schedule_dao::list().await?;
    println!("schedule len: {:?}", list_schedule.len());

    // This println! comes first
    println!("hello");
    op.await;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/users", get(list_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn list_user() -> (StatusCode, Json<UsersModel>) {
    // let list_user: Vec<users::Model> = user_dao::list().await;
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
        return (StatusCode::OK, Json(UsersModel {
            data: data
        }));
    };

    let res = UsersModel {
        data: vec![],
    };
    (StatusCode::BAD_REQUEST, Json(res))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
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
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Serialize)]
struct UsersModel {
    data: Vec<UserData>,
}

