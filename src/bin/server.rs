extern crate diesel;
extern crate marvin;

use marvin::*;

fn main() {
    println!("Hello, world!");

    let connection = establish_connection();
    // let test_package = package::NewPackage {
    //     name: "Test Package 01",
    //     description: None,
    //     homepage: None,
    //     documentation: None,
    //     repository: None,
    //     license: None,
    //     max_upload_size: None,
    // };
    //
    // test_package.create_or_update(&connection, 0);
    list_packages(&connection);
}
