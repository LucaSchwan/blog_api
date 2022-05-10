pub mod post;
pub mod user;

use super::db;
use super::schema;

pub use post::Post;
pub use post::NewPost;

pub use user::User;
pub use user::NewUser;
