#[macro_use] extern crate rocket;
extern crate serde_derive;

use blog_api::db::DbConn;
use blog_api::controller::routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes())
}

