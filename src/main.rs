#![feature(proc_macro_hygiene, decl_macro)]

mod router;
mod cors;
mod res;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use rocket::fairing::AdHoc;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;


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
