mod handler;

use axum::Router;
use axum::routing::{get, post};
use handler::{*};

pub async fn regist_route() -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(users::create_user))
        .route("/users", get(users::list_user))
        .route("/teachers", get(teachers::list_teacher))
        .route("/orders", get(orders::list_order));
    return app;
}
