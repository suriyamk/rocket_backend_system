#[macro_use] extern crate rocket;
use std::env;

use rocket::{get, launch, routes, Rocket, Build};
use diesel::{Connection, sqlite::SqliteConnection};
use dotenvy::dotenv;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASEURL").expect("Database url must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build>  {
    rocket::build().mount("/", routes![index])
}
