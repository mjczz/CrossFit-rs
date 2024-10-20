pub mod users;
pub mod teachers;
pub mod orders;
pub mod courses;

use users::{*};
use teachers::{*};
use orders::{*};
use courses::{*};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}
