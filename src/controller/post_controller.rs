use super::db::DbConn;
use super::models::{NewPost, Post};
use super::JsonMessage;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{self, Route};

pub fn routes() -> Vec<Route> {
    routes![
        get_post,
        create_post,
        update_post,
        delete_post,
        publish_post,
        get_all_posts,
        get_published_posts,
    ]
}

#[get("/post/<id>")]
async fn get_post(id: i32, conn: DbConn) -> Result<Json<Post>, Status> {
    let post = Post::read_one(id, &conn).await;

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::NotFound),
    }
}

#[post("/post", format = "json", data = "<new_post>")]
async fn create_post(new_post: Json<NewPost>, conn: DbConn) -> Result<Json<JsonMessage>, Status> {
    let create_post = NewPost {
        ..new_post.into_inner()
    };
    let rows_affected = Post::create(create_post, &conn).await;

    let rows_affected = match rows_affected {
        Ok(rows_affected) => rows_affected,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(JsonMessage {
        message: String::from("success"),
        rows_affected,
    }))
}

#[put("/post", format = "json", data = "<update_post>")]
async fn update_post(update_post: Json<Post>, conn: DbConn) -> Result<Json<Post>, Status> {
    let post = Post::update(update_post.into_inner(), &conn).await;

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::NotFound),
    }
}

#[delete("/post/<id>")]
async fn delete_post(id: i32, conn: DbConn) -> Result<Json<JsonMessage>, Status> {
    let rows_affected = Post::delete(id, &conn).await;

    let rows_affected = match rows_affected {
        Ok(rows_affected) => rows_affected,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(Json(JsonMessage {
        message: String::from("success"),
        rows_affected,
    }))
}

#[put("/post/<id>/publish")]
async fn publish_post(id: i32, conn: DbConn) -> Result<Json<Post>, Status> {
    let post = Post::publish(id, &conn).await;

    match post {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/posts/all")]
async fn get_all_posts(conn: DbConn) -> Result<Json<Vec<Post>>, Status> {
    let all_posts = Post::read_all(&conn).await;

    match all_posts {
        Ok(all_posts) => Ok(Json(all_posts)),
        Err(_) => Err(Status::NotFound),
    }
}

#[get("/posts/published")]
async fn get_published_posts(conn: DbConn) -> Result<Json<Vec<Post>>, Status> {
    let published_posts = Post::read_published(&conn).await;

    match published_posts {
        Ok(published_posts) => Ok(Json(published_posts)),
        Err(_) => Err(Status::NotFound),
    }
}
