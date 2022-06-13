#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
// #[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

use dotenv::dotenv;

mod config;
mod routes;
mod models;
mod schema;
mod db;

pub fn rocket() -> rocket::Rocket {
    dotenv().ok();
    rocket::custom(config::from_env())
        .manage(db::init_pool())
        .mount(
            "/wonky",
            routes![
                routes::health::check,
                routes::user::get
            ]
        )
}