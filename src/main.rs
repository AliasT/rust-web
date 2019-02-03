#![feature(proc_macro_hygiene, decl_macro)]

mod router;
mod cors;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use rocket::fairing::AdHoc;

#[macro_use]
extern crate rocket;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        // 跨域设置
        .attach(cors::CORS())
        .mount("/api", routes![
            router::user::signup
        ])
        .launch();
}
