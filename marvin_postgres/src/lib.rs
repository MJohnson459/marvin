#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, name: &'a str, status: &'a str) -> models::Package {
    let new_package = models::NewPackage { name, status };

    diesel::insert_into(schema::packages::table)
        .values(&new_package)
        .get_result(conn)
        .expect("Error saving new package")
}
