use super::schema::*;

pub enum RepositoryStatus {
    Maintained,
    Developed,
}

pub enum SupportedVcs {
    Git,
    Hg,
    Svn,
}

#[derive(Queryable)]
pub struct Documentation {
    id: i32,
    vcs: String,
    url: String,
    version: String,
}

#[derive(Insertable)]
#[table_name="documentation"]
pub struct NewDocumentation<'a> {
    vcs: &'a str,
    url: &'a str,
    version: &'a str,
}


#[derive(Insertable, Queryable)]
#[table_name="packages"]
pub struct Package {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name="packages"]
pub struct NewPackage {
    name: String,
}

#[derive(Queryable)]
pub struct Release {
    id: i32,
    tags: Option<String>,
    url: Option<String>,
    version: String,
}

#[derive(Insertable)]
#[table_name="releases"]
pub struct NewRelease<'a> {
    tags: &'a str,
    url: &'a str,
    version: &'a str,
}

#[derive(Insertable, Queryable)]
#[table_name="sources"]
pub struct Source {
    id: i32,
    vcs: String,
    url: String,
    version: String,
}

#[derive(Associations, Insertable, Identifiable, Debug, Clone, Copy)]
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
