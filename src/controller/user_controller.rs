use super::db::DbConn;
use super::models::{NewUser, User};
use super::JsonMessage;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{self, Route};

pub fn routes() -> Vec<Route> {
    routes![
        get_user,
        create_user,
        update_user,
        delete_user,
    ]
}

#[get("/user/<id>")]
async fn get_user(id: i32, conn: DbConn) -> Result<Json<User>, Status> {
    let user = User::get_one(id, &conn).await;

    match user {
	Ok(user) => Ok(Json(user)),
	Err(_) => Err(Status::NotFound)
    }
}

#[post("/user", format = "json", data = "<create_user>")]
async fn create_user(create_user: Json<NewUser>, conn: DbConn) -> Result<Json<JsonMessage>, Status> {
    let rows_affected = User::create(create_user.into_inner(), &conn).await;

    let rows_affected = match rows_affected {
	Ok(rows_affected) => rows_affected,
	Err(_) => return Err(Status::InternalServerError)
    };

    Ok(Json(JsonMessage {
	message: String::from("success"),
	rows_affected
    }))
}

#[put("/user", format = "json", data = "<update_user>")]
async fn update_user(update_user: Json<User>, conn: DbConn) -> Result<Json<User>, Status> {
    let user = User::update(update_user.into_inner(), &conn).await;

    match user {
	Ok(user) => Ok(Json(user)),
	Err(_) => Err(Status::InternalServerError)
    }
}

#[delete("/user/<id>")]
async fn delete_user(id: i32, conn: DbConn) -> Result<Json<JsonMessage>, Status> {
    let rows_affected = User::delete(id, &conn).await;

    let rows_affected = match rows_affected {
	Ok(rows_affected) => rows_affected,
	Err(_) => return Err(Status::NotFound)
    };

    Ok(Json(JsonMessage {
	message: String::from("success"),
	rows_affected
    }))
}
