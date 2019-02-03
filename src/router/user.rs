
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use std::collections::HashMap;
use crate::db::user::{User, NewUser};
use crate::schema::users;
use diesel;
use diesel::prelude::*;

#[post("/signup", data="<user>")]
pub fn signup(conn: crate::DbConn, user: Form<NewUser>) -> JsonValue {
    NewUser::create(&conn, user.username.clone());

    crate::res::SuccessResponse::new("3")
}
