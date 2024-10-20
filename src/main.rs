mod entities;
mod dao;
mod route;

use sea_orm::{DbErr};
use crate::dao::{*};
use crate::entities::{*};
use std::{thread, time};
use serde::{Deserialize, Serialize};
use crate::route::regist_route;

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

    // let list_teacher: Vec<teachers::Model> = teacher_dao::list().await?;
    // for teacher in list_teacher.iter() {
    //     println!("{:?} : {:?}", teacher.name, teacher.teacher_id);
    //     println!("");
    // }
    // println!("teacher len: {:?}", list_teacher.len());

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
    let app = regist_route().await;

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}



