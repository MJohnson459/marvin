use chrono::{NaiveDate, NaiveDateTime};
use diesel::associations::Identifiable;
use diesel::prelude::*;
use diesel;
use semver;
use url::Url;

use schema::*;

#[derive(Debug, Clone, Queryable)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub downloads: i32,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub max_upload_size: Option<i32>,
}

type AllColumns = (
    packages::id,
    packages::name,
    packages::updated_at,
    packages::created_at,
    packages::downloads,
    packages::description,
    packages::homepage,
    packages::documentation,
    packages::license,
    packages::repository,
    packages::max_upload_size,
);

pub const ALL_COLUMNS: AllColumns = (
    packages::id,
    packages::name,
    packages::updated_at,
    packages::created_at,
    packages::downloads,
    packages::description,
    packages::homepage,
    packages::documentation,
    packages::license,
    packages::repository,
    packages::max_upload_size,
);

pub const MAX_NAME_LENGTH: usize = 64;

type All = diesel::dsl::Select<packages::table, AllColumns>;

#[derive(Debug, Default, Insertable)]
#[table_name="packages"]
pub struct NewPackage<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub homepage: Option<&'a str>,
    pub documentation: Option<&'a str>,
    pub license: Option<&'a str>,
    pub repository: Option<&'a str>,
    pub max_upload_size: Option<i32>,
}
