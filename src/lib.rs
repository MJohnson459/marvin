#![recursion_limit = "128"]

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate diesel_full_text_search;
extern crate dotenv;
extern crate semver;
extern crate url;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use models::*;

pub mod models;
pub mod package;
pub mod schema;
pub mod util;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn list_packages(conn: &PgConnection) {
    use schema::packages::dsl::*;
    use package;

    let query = packages.select(package::ALL_COLUMNS);

    let result = query.load::<Package>(conn).expect("Error loading packages");


    for package in result.iter() {
        println!("Package: {:?}", package);
    }

    println!("Displaying {} packages", result.len());
}
