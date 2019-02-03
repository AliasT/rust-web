
use crate::schema::users;
use crate::DbConn;
use diesel;
use diesel::RunQueryDsl;

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
      let new_user = NewUser {
        username
      };
      diesel::insert_into(users::table)
      .values(&new_user)
      .get_result(conn)
      .expect("Error saving new post")
  }
}


impl User {
  fn create(conn: &DbConn, user: User) {

  }
}
