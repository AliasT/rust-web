use rocket::request::Form;

#[derive(FromForm)]
pub struct User {
    username: String,
    password: String,
}

#[post("/signup", data="<user>")]
pub fn signup(user: Form<User>) -> &'static str {
    "signup success"
}
