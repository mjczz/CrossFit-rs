mod handler;

use axum::Router;
use axum::routing::{get, post};
use serde::Serialize;
use handler::{*};

#[macro_export]
macro_rules! common_fields {
    ($name:ident, $tt:ty, $count:expr) => {
        #[derive(Serialize, Default)]
        pub struct $name{
            pub code: usize,
            pub msg: String,
            pub count: usize,
            data: Vec<$tt>,
        }
        impl $name {
            pub fn new(data: Vec<$tt>) -> Self {
                Self {
                    code: 0,
                    msg: String::from("Success"),
                    count: data.len(),
                    data,
                }
            }
        }
    };
    ($name:ident, $tt:ty) => {
        #[derive(Serialize, Default)]
        pub struct $name{
            code: usize,
            msg: String,
            data: Vec<$tt>,
        }
    };
}


pub async fn regist_route() -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(users::create_user))
        .route("/users", get(users::list))
        .route("/teachers", get(teachers::list))
        .route("/courses", get(courses::list))
        .route("/schedules", get(schedules::list))
        .route("/orders", get(orders::list));
    return app;
}
