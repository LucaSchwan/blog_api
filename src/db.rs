use rocket_sync_db_pools::diesel;

#[database("blog_api")]
pub struct DbConn(diesel::PgConnection);
