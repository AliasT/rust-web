
use crate::schema::users;
use diesel;
use diesel::RunQueryDsl;
use diesel::prelude::*;

#[derive(Queryable, FromForm, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String
}

#[derive(Insertable, FromForm, Serialize)]
#[table_name="users"]
pub struct NewUser {
    pub username: String
}

impl NewUser {
    pub fn create(conn: &diesel::PgConnection, username: String) -> User {
        let new_user = NewUser { username };
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
