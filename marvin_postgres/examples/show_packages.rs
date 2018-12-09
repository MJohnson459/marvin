extern crate diesel;
extern crate marvin_postgres;

use marvin_postgres::schema::packages::dsl::*;
use marvin_postgres::*;

use diesel::prelude::*;

fn main() {
    let connection = establish_connection();
    let results = packages
        .filter(status.eq("maintained"))
        .limit(5)
        .load::<models::Package>(&connection)
        .expect("Error loading packages");

    println!("Displaying {} packages", results.len());
    for package in results {
        println!("{:?}", package);
    }
}
