#[macro_use]
extern crate rocket;

use rocket::fairing::{self, AdHoc};
use rocket::serde::json::{json, Value};
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

mod bootstrap;
pub mod controller;
pub mod model;
pub mod data;
use bootstrap::{db_migrations, db_pool};


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &db_pool::Db::fetch(&rocket).unwrap().conn;
    let _ = db_migrations::create_tables(conn).await;
    Ok(rocket)
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn bootstrap() -> Rocket<Build> {
    use figment::{
        providers::{Env, Format, Toml},
        Figment,
    };

    let figment = Figment::new()
        .merge(rocket::Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("ROCKET_APP_").split("_"));

    rocket::custom(figment)
        .attach(db_pool::Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/logs", controller::logs::routes())
        .register("/", catchers![not_found])
}
