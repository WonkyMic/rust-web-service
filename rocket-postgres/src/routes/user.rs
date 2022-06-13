use rocket::http::RawStr;
use crate::db::DbConn;
use crate::db::crud;
use std::str::FromStr;
use rocket_contrib::{json, json::{JsonValue}};


#[get("/user/<id>")]
pub fn get(conn: DbConn, id: &RawStr) -> Option<JsonValue> {
    crud::find(&conn, FromStr::from_str(id).unwrap())
        .map(|user| json!({"handle": user.handle}))
}