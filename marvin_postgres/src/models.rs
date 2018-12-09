use super::schema::packages;

#[derive(Debug, Queryable)]
pub struct Package {
    id: i32,
    name: String,
    status: String,
}

#[derive(Insertable)]
#[table_name = "packages"]
pub struct NewPackage<'a> {
    pub name: &'a str,
    pub status: &'a str,
}

#[derive(Queryable)]
pub struct SubPackage {
    id: i32,
    package_id: i32,
    name: String,
}

#[derive(Queryable)]
pub struct Version {
    id: i32,
    package_id: i32,
    num: String,
}

#[derive(Queryable)]
pub struct Documentation {
    id: i32,
    package_id: i32,
    vcs: String,
    url: String,
    version: String,
}

#[derive(Queryable)]
pub struct Source {
    id: i32,
    package_id: i32,
    vcs: String,
    url: String,
    version: String,
}
