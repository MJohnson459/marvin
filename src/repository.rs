extern crate serde_yaml;

use std::collections::BTreeMap as Map;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use super::schema::*;

#[derive(Serialize, Deserialize, Queryable, Debug, Default)]
pub struct Documentation {
    #[serde(skip_serializing)]
    pub id: i32,
    #[serde(rename = "type")]
    pub vcs: String,
    pub url: String,
    pub version: String,
}

#[derive(Insertable)]
#[table_name="documentation"]
pub struct NewDocumentation<'a> {
    pub vcs: &'a str,
    pub url: &'a str,
    pub version: &'a str,
}

#[derive(Queryable, Debug, Default)]
pub struct Tag {
    pub id: i32,
    pub release_id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Queryable, Debug, Default)]
pub struct Release {
    pub id: i32,
    pub url: String,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EncodableRelease {
    pub packages: Option<Vec<String>>,
    pub tags: Map<String, String>,
    pub url: String,
    pub version: Option<String>,
}

#[derive(Insertable)]
#[table_name="releases"]
pub struct NewRelease<'a> {
    pub url: &'a str,
    pub version: &'a str,
}

#[derive(Queryable, Debug, Default)]
pub struct Source {
    pub id: i32,
    pub vcs: String,
    pub url: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EncodableSource {
    #[serde(rename = "type")]
    pub vcs: String,
    pub url: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EncodableRepository {
    pub doc: Option<Documentation>,
    pub release: Option<EncodableRelease>,
    pub source: Option<Source>,
    pub status: Option<String>,
}


#[derive(Queryable)]
pub struct Package {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewPackage<'a> {
    name: &'a str,
}

#[derive(Insertable)]
#[table_name="sources"]
pub struct NewSource<'a> {
    vcs: &'a str,
    url: &'a str,
    version: &'a str,
}

#[derive(Associations, Insertable, Queryable, Identifiable, Debug, Clone, Copy)]
#[primary_key(package_id, release_id)]
#[table_name="package_releases"]
pub struct PackageRelease {
    package_id: i32,
    release_id: i32,
}

#[derive(Queryable)]
pub struct Repository {
    name: String,
    documentation: Option<Documentation>,
    release: Option<Release>,
    source: Option<Source>,
    status: Option<String>,
}

pub fn create_or_update() {
    println!("TODO");
}

