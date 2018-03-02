#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate marvin;
extern crate rocket;
extern crate rocket_contrib;

use marvin::*;
use marvin::views::EncodablePackage;
use rocket_contrib::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/list")]
fn list() -> Option<Json<EncodablePackage>> {
    let connection = establish_connection();
    let packages = list_packages(&connection);

    println!("Retrieved {} packages", packages.len());

    if packages.len() > 0 {
        let first_package = packages[0].clone();
        Some(rocket_contrib::Json(first_package.minimal_encodable()))
    } else {
        None
    }
}

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
    let result = list_packages(&connection);

    for package in result.iter() {
        println!("Package: {:?}", package);
    }

    println!("Displaying {} packages", result.len());

    rocket::ignite().mount("/", routes![index, list]).launch();
}
