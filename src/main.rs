mod entities;
mod dao;

use sea_orm::{DbErr};
use crate::dao::{*};
use crate::entities::{*};

// 使用 tokio::main 来启动异步运行时
#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let list_teacher: Vec<teachers::Model> = teacher_dao::list().await?;
    for teacher in list_teacher.iter() {
        println!("{:?} : {:?}", teacher.name, teacher.teacher_id);
        println!("");
    }
    println!("teacher len: {:?}", list_teacher.len());

    let list_user: Vec<users::Model> = user_dao::list().await?;
    println!("user len: {:?}", list_user.len());

    let list_order: Vec<orders::Model> = order_dao::list().await?;
    println!("order len: {:?}", list_order.len());

    let list_course: Vec<courses::Model> = course_dao::list().await?;
    println!("course len: {:?}", list_course.len());

    let list_schedule: Vec<schedules::Model> = schedule_dao::list().await?;
    println!("schedule len: {:?}", list_schedule.len());

    Ok(())
}