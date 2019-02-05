#![feature(proc_macro_hygiene, decl_macro)]


mod router;
mod cors;
mod res;
mod db;
mod schema;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use rocket::fairing::AdHoc;

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);


fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        // 跨域设置
        .attach(cors::CORS())
        .mount("/api", routes![
            router::user::signup,
            router::user::get_all_users
        ])
        .launch();
}
