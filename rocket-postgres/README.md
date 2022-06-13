# Rust web-service
A web service fronting a relational database.

## Resources
### Guide
- [Genekuo - REST API with Persistence](https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104)
- [realworld-rust-rocket](https://github.com/TatriX/realworld-rust-rocket) (github)
### Rust
- [Rocket, a web framework](https://api.rocket.rs/)
- [Juniper, a GraphQL framework](https://docs.rs/juniper)
- [Diesel, ORM and Query Builder](https://docs.diesel.rs/)
- [dotenv, load environment variables from a .env file](https://crates.io/crates/dotenv)
### Database
- [Postgresql, relational database](https://www.postgresql.org/)

## Database Connection Pool Options
### rocket_contrib
- [Rust Docs](https://api.rocket.rs/v0.4/rocket_contrib/databases/index.html)

**Preferred.** Since we're already using Rocket as a web framework, let's try using their connection pooling as well. This may lead to easier lifecycle management.

_rocket_contrib_ is also less assuming. We may not have to schema management in the project like our other option Diesel.

It appears that Diesel is still used within this lib as a lower level dependency.

#### Supported Drivers
- deadpool(0.8) - **Preferred**
  - Postgres
  - Redis << **This will determine our cache type as well**
- sqlx(v0.5)
- mongodb(v1)

### Diesel
A safe, extensible ORM and Query Builder for Rust. 

Diesel provides a separate [CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) tool to help manage your project. Since it's a standalone binary, and doesn't affect your project's code directly, we don't add it to **Cargo.toml**. Instead, we just install it on our system.

rocket_contrib = { version = "0.4.10", default_features = false, features = ["diesel_postgres_pool"] }

## Additional Resources
- [Learning Makefiles](http://swcarpentry.github.io/make-novice/)
- [Cargo Commands](https://doc.rust-lang.org/cargo/commands)
- [Dockerfile : multi-stage builds](https://docs.docker.com/develop/develop-images/multistage-build/)
  - This pattern aims to keep the image layer size as small as possible and ensures our image contins only what is needed to run.
- [Postgres Cheat Sheet](https://postgrescheatsheet.com/)