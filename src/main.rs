#[macro_use] extern crate rocket;
extern crate redis;

use redis::Commands;
use rocket::serde::{Deserialize, json::Json};
use rocket::response::status;
use rocket::http::Status;

// Transfer Object
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct User<'r> {
    id: &'r str,
    name: &'r str
}

// controller
#[get("/health")]
fn health() -> &'static str {
    "Application Healthy"
}

#[post("/user", data="<user>")]
fn upsert_user(user: Json<User<'_>>) -> status::Custom<&'static str> {
    println!("Upsert user.id::{}", user.id);
    println!("Upsert user.name::{}", user.name);
    upsert(user.into_inner()).unwrap();
    status::Custom(Status::Accepted, "upsert user")
}

#[get("/user/<id>")]
fn user(id: &str) -> String {
    let client = redis::Client::open("CONN_STRING").unwrap();
    let mut conn = client.get_connection().expect("Redis Read connection");
    let result: String = conn.get(id).unwrap();
    format!("Hello, {}!", result)
}

// Service
fn upsert(user: User) -> redis::RedisResult<()> {
    let client = redis::Client::open("CONN_STRING")?;
    let mut conn = client.get_connection().expect("Redis Write connection");
    let _ : () = conn.set(user.id, user.name)?;
    Ok(())
}

// main
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, user, upsert_user])
}