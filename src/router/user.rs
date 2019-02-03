
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use std::collections::HashMap;

#[derive(Serialize, FromForm)]
pub struct User {
    username: String,
    password: String,
}

#[post("/signup", data="<user>")]
pub fn signup(user: Form<User>) -> JsonValue {
  let mut r = HashMap::new();
  r.insert("m", "hello world");
  crate::res::SuccessResponse::new(r)
}
