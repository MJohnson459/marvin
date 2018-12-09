extern crate diesel;
extern crate marvin_postgres;

use marvin_postgres::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = &args[1];
    let status = &args[2];

    let connection = establish_connection();

    let package = create_post(&connection, &name, &status);
    println!("\nSaved package {:?}", package);
}
