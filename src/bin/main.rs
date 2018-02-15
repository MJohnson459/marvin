extern crate marvin;
extern crate diesel;

use marvin::*;

fn main() {
    println!("Hello, world!");

    let connection = establish_connection();
    list_packages(&connection);
    list_documentation(&connection);
}
