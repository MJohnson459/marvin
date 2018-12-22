#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate marvin_core;
extern crate ros_parser;

use std::path::Path;

use marvin_core::PackageReader;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let path = Path::new("./external/rosdistro/melodic/distribution.yaml");
    let store = ros_parser::load_distribution(&path.to_str().unwrap()).unwrap();

    rocket::ignite().mount("/", routes![index]).launch();
}