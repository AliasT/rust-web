#![feature(proc_macro_hygiene, decl_macro)]

mod cors;
mod db;
mod res;
mod router;
mod schema;

use rocket_contrib::templates::Template;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

#[database("postgres")]
pub struct DbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        // 跨域设置
        .attach(cors::CORS())
        .mount(
            "/api",
            routes![router::user::signup, router::user::get_all_users],
        )
        .launch();
}
