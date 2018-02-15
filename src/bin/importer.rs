#[macro_use]
extern crate serde_derive;

extern crate diesel;
extern crate marvin;
extern crate serde;
extern crate serde_yaml;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::collections::BTreeMap as Map;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

use marvin::*;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EncodableDistribution {
    release_platforms: Map<String, Vec<String>>,
    repositories: Map<String, EncodableRepository>,
    #[serde(rename = "type")]
    type_name: String,
    version: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableDocumentation {
    #[serde(rename = "type")]
    vcs: String,
    url: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableTag {
    release: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableRelease {
    packages: Option<Vec<String>>,
    tags: EncodableTag,
    url: String,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableSource {
    #[serde(rename = "type")]
    vcs: String,
    url: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableRepository {
    doc: Option<EncodableDocumentation>,
    release: Option<EncodableRelease>,
    source: Option<EncodableSource>,
    status: Option<String>,
}


pub fn load_distribution(path: &str) -> Result<EncodableDistribution, String> {
    let contents = read_distribution(path).unwrap();


    let distribution: EncodableDistribution = serde_yaml::from_str(&contents).unwrap();

    Ok(distribution)
}

pub fn read_distribution(path: &str) -> io::Result<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn insert_documentation(conn: &PgConnection, documentation: &Option<EncodableDocumentation>) -> Option<i32> {
    use schema::documentation;
    use models::{Documentation, NewDocumentation};
    if let Some(ref doc) = *documentation {
        let new_doc = NewDocumentation {
            vcs: doc.vcs.as_str(),
            url: doc.url.as_str(),
            version: doc.version.as_str(),
        };

        let result: diesel::QueryResult<Documentation> = diesel::insert_into(documentation::table)
            .values(&new_doc)
            .get_result(conn);

        match result {
                Ok(val) => Some(val.id),
                Err(_) => None,
        };
    }

    None
}

pub fn push_distribution(conn: &PgConnection, distribution: &EncodableDistribution) {

    // TODO switch to batch import
    for (key, value) in distribution.repositories.iter() {
        let new_doc = insert_documentation(conn, &value.doc);

    }

    //diesel::insert_into(documentation::table)
        //.values(&new_docs)
        //.get_result(conn)
        //.expect("Error saving new documentation")
}

fn main() {
    // TODO error handling needed
    let path = Path::new("./external/rosdistro/kinetic/distribution.yaml");
    println!("Path = {:?}", fs::canonicalize(&path));
    let distribution = load_distribution(&path.to_str().unwrap()).unwrap();
    //println!("distribution = {:#?}", distribution);

    let connection = establish_connection();

    push_distribution(&connection, &distribution);
}
