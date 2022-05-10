use super::db::DbConn;
use super::schema::posts;
use super::schema::posts::published;
use diesel::{self, prelude::*};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

impl Post {
    pub async fn create(new_post: NewPost, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::insert_into(posts::table)
                .values(&new_post)
                .execute(c)
        })
        .await
    }

    pub async fn read_all(conn: &DbConn) -> QueryResult<Vec<Post>> {
        conn.run(|c| posts::table.order(posts::id.asc()).load::<Post>(c))
            .await
    }

    pub async fn read_published(conn: &DbConn) -> QueryResult<Vec<Post>> {
        conn.run(|c| {
            posts::table
                .filter(published.eq(true))
                .order(posts::id.asc())
                .load::<Post>(c)
        })
        .await
    }

    pub async fn read_one(id: i32, conn: &DbConn) -> QueryResult<Post> {
        conn.run(move |c| posts::table.find(id).get_result::<Post>(c))
            .await
    }

    pub async fn publish(id: i32, conn: &DbConn) -> QueryResult<Post> {
        conn.run(move |c| {
            diesel::update(posts::table.find(id))
                .set(published.eq(true))
                .get_result::<Post>(c)
        })
        .await
    }

    pub async fn update(post: Post, conn: &DbConn) -> QueryResult<Post> {
        conn
            .run(move |c| {
                diesel::update(posts::table.find(post.id))
                    .set(&post)
                    .get_result::<Post>(c)
            })
            .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
	conn.run(move |c| {
	    diesel::delete(posts::table.find(id)).execute(c)
	}).await
    }
}
