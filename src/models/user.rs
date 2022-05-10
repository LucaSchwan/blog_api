use super::db::DbConn;
use super::schema::users;
use diesel::{self, prelude::*};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub async fn create(new_user: NewUser, conn: &DbConn) -> QueryResult<usize> {
        // hash password
        conn.run(move |c| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(c)
        })
        .await
    }

    pub async fn get_one(id: i32, conn: &DbConn) -> QueryResult<User> {
        conn.run(move |c| users::table.find(id).get_result::<User>(c))
            .await
    }

    pub async fn update(user: User, conn: &DbConn) -> QueryResult<User> {
        conn.run(move |c| {
            diesel::update(users::table.find(user.id))
                .set(&user)
                .get_result::<User>(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(users::table.find(id)).execute(c))
            .await
    }
}
