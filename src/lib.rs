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

//use models::*;

//pub fn create_documentation<'a>(conn: &PgConnection, vcs: &'a str, url: &'a str, version: &'a str) -> Documentation {
    //use schema::documentation;

    //let new_docs = NewDocumentation {
        //vcs: vcs,
        //url: url,
        //version: version,
    //};

    //diesel::insert_into(documentation::table)
        //.values(&new_docs)
        //.get_result(conn)
        //.expect("Error saving new documentation")
//}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Distribution {
    release_platforms: Map<String, Vec<String>>,
    repositories: Map<String, Repository>,
    #[serde(rename = "type")]
    type_name: String,
    version: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Documentation {
    #[serde(rename = "type")]
    vcs: String,
    url: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Tag {
    release: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Release {
    packages: Option<Vec<String>>,
    tags: Tag,
    url: String,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Source {
    #[serde(rename = "type")]
    vcs: String,
    url: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Repository {
    doc: Option<Documentation>,
    release: Option<Release>,
    source: Option<Source>,
    status: Option<String>,
}


pub fn load_distribution(path: &str) -> Result<Distribution, String> {
    let contents = read_distribution(path).unwrap();


    let distribution: Distribution = serde_yaml::from_str(&contents).unwrap();

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

pub fn add_repository() {

    let new_repo = Repository {
        doc: Some(Documentation::default()),
        release: Some(Release::default()),
        source: Some(Source::default()),
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

    let deserialized: Repository = serde_yaml::from_str(&y_rep).unwrap();

    //println!("deserialized = {:#?}", deserialized);
}

