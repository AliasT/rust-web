
use rocket::request::Form;
use rocket_contrib::json::{JsonValue};
use crate::db::user::{User, NewUser};
use crate::res::SuccessResponse;

#[post("/signup", data="<user>")]
pub fn signup(conn: crate::DbConn, user: Form<NewUser>) -> JsonValue {
    let new_user = NewUser::create(&conn, user.into_inner());
    SuccessResponse::new(new_user)
}

#[get("/users")]
pub fn get_all_users(conn: crate::DbConn) -> JsonValue {
    SuccessResponse::new(User::get(&conn))
}
