pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::collections::BTreeMap as Map;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io;

use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn list_packages(conn: &PgConnection) {
    use schema::packages::dsl::*;

    let result = packages
        .load::<Package>(conn)
        .expect("Error loading packages");

    println!("Displaying {} packages", result.len());
}

pub fn list_documentation(conn: &PgConnection) {
    use schema::documentation::dsl::*;

    let result = documentation
        .load::<Documentation>(conn)
        .expect("Error loading packages");

    println!("Displaying {} documents", result.len());
}
