extern crate rocket;

pub mod crud;

use std::ops::Deref;
use std::env::var;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
// use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

// Type alias
type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref DATABASE_URL: String = var("DATABASE_URL").expect("No DATABASE_URL found");
}
pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

pub fn init_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(&*DATABASE_URL);
    Pool::new(manager).expect("db pool")
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn)  => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}