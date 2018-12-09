
pub struct Package {
    id: i32,
    name: String,
    status: String,
}

pub trait PackageReader {
    fn get_package(id: i32) -> Package;
    fn search_package_name(name: &str) -> Package;
}

pub trait PackageWriter {
    fn save_package(name: &str, status: &str) -> Package;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
