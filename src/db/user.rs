use crate::schema::users;
use diesel;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use base64::{encode};

#[derive(Queryable, FromForm, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub password: String,
}

#[derive(Serialize, FromForm, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(FromForm, Debug)]
pub struct SignupUser {
    pub username: String,
    pub password: String,
    pub password_confirmation: String
}

impl NewUser {
    pub fn create(conn: &diesel::PgConnection, user: SignupUser) -> User {
        let new_user = NewUser {
            username: user.username,
            password: encode(&user.password) ,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .expect("Error saving new post")
    }
}

impl User {
    pub fn get(conn: &diesel::PgConnection) -> Vec<User> {
        use crate::schema::users::dsl::*;
        users
            .limit(5)
            .load::<User>(conn)
            .expect("Error loading posts")
    }
}
