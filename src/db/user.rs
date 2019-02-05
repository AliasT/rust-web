
use crate::schema::users;
use diesel;
use diesel::RunQueryDsl;
use diesel::prelude::*;

#[derive(Queryable, FromForm, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub nickname: String,
}

#[derive(Insertable, FromForm, Serialize, Queryable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub nickname: String
}

impl NewUser {
    pub fn create(conn: &diesel::PgConnection, user: NewUser) -> User {
        diesel::insert_into(users::table)
            .values(&user)
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
