extern crate marvin;
extern crate diesel;

use marvin::*;
use std::fs;
use std::path::Path;

fn main() {
    println!("Hello, world!");
    //let connection = establish_connection();

    // TODO error handling needed
    let path = Path::new("./external/rosdistro/kinetic/distribution.yaml");
    println!("Path = {:?}", fs::canonicalize(&path));
    let distribution = importer::load_distribution(&path.to_str().unwrap()).unwrap();
    //println!("distribution = {:#?}", distribution);
}
