#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;

#[macro_use]
extern crate rocket;
#[get("/hello/<name>")]
fn index(name: String) -> Template {
    let mut context = HashMap::new();
    context.insert(String::from("name"), 10);
    Template::render("index", &context)
}

#[derive(FromForm)]
struct User {
    username: String,
    password: String,
}

#[post("/signup", data="<user>")]
fn signup(user: Form<User>) -> &'static str {
    "signup success"
}

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![
        index,
        signup
    ]).launch();

}
