pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

use std::collections::BTreeMap as Map;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


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

    for (key, value) in distribution.repositories.iter() {
        let new_doc = insert_documentation(conn, &value.doc);

    }

    //diesel::insert_into(documentation::table)
        //.values(&new_docs)
        //.get_result(conn)
        //.expect("Error saving new documentation")
}

pub fn add_repository() {

    let new_repo = EncodableRepository {
        doc: Some(EncodableDocumentation::default()),
        release: Some(EncodableRelease::default()),
        source: Some(EncodableSource::default()),
        status: Some("whatup".to_string()),
    };

    let serialized = serde_yaml::to_string(&new_repo).unwrap();

    //println!("serialized = {}", serialized);


    let y_rep = "\
  ---
  doc:
    type: git
    url: https://github.com/ros-industrial/abb.git
    version: kinetic-devel
  release:
    packages:
    - abb
    - abb_driver
    - abb_irb2400_moveit_config
    - abb_irb2400_moveit_plugins
    - abb_irb2400_support
    - abb_irb4400_support
    - abb_irb5400_support
    - abb_irb6600_support
    - abb_irb6640_moveit_config
    - abb_irb6640_support
    - abb_resources
    tags:
      release: release/kinetic/{package}/{version}
    url: https://github.com/ros-industrial-release/abb-release.git
    version: 1.3.0-1
  source:
    type: git
    url: https://github.com/ros-industrial/abb.git
    version: kinetic
  status: developed";

    let deserialized: EncodableRepository = serde_yaml::from_str(&y_rep).unwrap();

    //println!("deserialized = {:#?}", deserialized);
}

