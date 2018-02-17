extern crate diesel;
extern crate marvin;

use marvin::*;

fn main() {
    println!("Hello, world!");

    let connection = establish_connection();
    list_packages(&connection);
    list_documentation(&connection);
}
