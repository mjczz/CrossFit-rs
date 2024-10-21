pub mod users;
pub mod teachers;
pub mod orders;
pub mod courses;
pub mod schedules;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}
