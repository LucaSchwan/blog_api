#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate rocket;

pub mod schema;
pub mod models;
pub mod db;
pub mod controller;

