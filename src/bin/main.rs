extern crate marvin;
extern crate diesel;

use marvin::*;
use std::fs;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    //let connection = establish_connection();

    //let vcs = "Git";
    //let version = "1.4.6";
    //let url = "https://fast.com";

    //let document = create_documentation(&connection, vcs, url, version);
    //println!("\nSaved document with id {:?}", document);
    add_repository();

    // TODO error handling needed
    let path = Path::new("./external/rosdistro/kinetic/distribution.yaml");
    println!("Path = {:?}", fs::canonicalize(&path));
    let distribution = load_distribution(&path.to_str().unwrap()).unwrap();
    println!("distribution = {:#?}", distribution);
}
