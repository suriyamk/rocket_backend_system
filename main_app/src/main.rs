#[macro_use] extern crate rocket;
use std::env;

use diesel::{Connection, sqlite::SqliteConnection};
use dotenvy::dotenv;
fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASEURL").expect("Database url must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}