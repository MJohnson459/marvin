#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

extern crate marvin_core;

use std::collections::BTreeMap as Map;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

use marvin_core::{Package, PackageReader};

fn main() {
    // TODO error handling needed
    let path = Path::new("../external/rosdistro/kinetic/distribution.yaml");
    println!("Path = {:?}", fs::canonicalize(&path));
    let distribution = load_distribution(&path.to_str().unwrap()).unwrap();
    let package_list = distribution.to_package_list();

    for package in package_list.iter() {
        println!("Package: {:?}", package);
    }
}

fn publish_package_list(conn: &PgConnection, package_list: &Vec<NewPackage>) {
    // TODO use batch import
    use schema::packages;

    diesel::insert_into(packages::table)
        .values(package_list)
        .execute(conn)
        .expect("Error saving new packages");
}

