#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub status: String,
}

pub trait PackageReader {
    fn search_package_name(&self, name: &str) -> Option<Package>;
}

pub trait PackageWriter {
    fn save_package(&mut self, name: &str, status: &str) -> Package;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
