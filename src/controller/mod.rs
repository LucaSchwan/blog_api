mod post_controller;
mod user_controller;

use super::db;
use super::models;
use rocket::Route;

use rocket::serde::Serialize;

pub fn routes() -> Vec<Route> {
    let mut routes = post_controller::routes();
    routes.append(&mut user_controller::routes());

    routes
}

#[derive(Serialize)]
pub struct JsonMessage {
    pub message: String,
    pub rows_affected: usize,
}
