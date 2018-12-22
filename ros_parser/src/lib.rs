//! Implements `marvin_cor::PackageReader` for the ROS distribution.yaml files
//!
//! http://www.ros.org/reps/rep-0141.html

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

extern crate marvin_core;

use std::collections::BTreeMap as Map;
use std::io;

use marvin_core::{Package, PackageReader};

pub fn load_distribution(path: &str) -> Result<Distribution, String> {
    let contents = read_distribution(path).unwrap();

    let distribution: Distribution = serde_yaml::from_str(&contents).unwrap();

    Ok(distribution)
}

fn read_distribution(path: &str) -> io::Result<String> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

impl PackageReader for Distribution {
    fn search_package_name(&self, search_name: &str) -> Option<Package> {
        for (repo_name, repository) in self.repositories.iter() {
            if repo_name == search_name {
                // The distribution.yml format is very relaxed. Most have a status but
                // not all and so we need to check before returning.
                if repository.status.is_some() {
                    return Some(Package {
                        name: repo_name.to_string(),
                        status: repository.status.clone().unwrap(),
                    });
                }
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Distribution {
    release_platforms: Map<String, Vec<String>>,
    repositories: Map<String, EncodableRepository>,
    #[serde(rename = "type")] type_name: String,
    version: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableDocumentation {
    #[serde(rename = "type")] vcs: String,
    url: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableTag {
    release: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableRelease {
    #[serde(default)] packages: Vec<String>,
    tags: EncodableTag,
    url: String,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct EncodableSource {
    #[serde(rename = "type")] vcs: String,
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


// fn create_new_package<'a>(
//     name: &'a str,
//     doc_url: Option<&'a str>,
//     source_url: Option<&'a str>,
// ) -> NewPackage<'a> {
//     NewPackage {
//         name: name,
//         description: None,
//         homepage: None,
//         documentation: doc_url,
//         license: None,
//         repository: source_url,
//         max_upload_size: None,
//     }
// }

// impl Distribution {
//     fn to_package_list(&self) -> Vec<Package> {
//         // name of repo == package
//         // if release.packages is not empty create list of packages

//         let mut package_list: Vec<Package> = Vec::new();

//         for (name, repository) in self.repositories.iter() {
//             let doc_url = if let Some(ref documentation) = repository.doc {
//                 Some(documentation.url.as_str())
//             } else {
//                 None
//             };

//             let source_url = if let Some(ref source) = repository.source {
//                 Some(source.url.as_str())
//             } else {
//                 None
//             };

//             if let Some(ref release) = repository.release {
//                 if !release.packages.is_empty() {
//                     // Multiple packages in the repo
//                     // One must be named the same as the repo
//                     for package in release.packages.iter() {
//                         package_list.push(create_new_package(package, doc_url, source_url));
//                     }
//                 } else {
//                     // Only one package
//                     package_list.push(create_new_package(name, doc_url, source_url));
//                 }
//             } else {
//                 // Only one package
//                 package_list.push(create_new_package(name, doc_url, source_url));
//             }
//         }

//         package_list
//     }
// }



#[cfg(test)]
mod tests {
    #[test]
    fn search_package_name() {
        use super::*;
        use std::path::Path;

        let path = Path::new("../external/rosdistro/melodic/distribution.yaml");
        let store = load_distribution(&path.to_str().unwrap()).unwrap();

        // Exist
        assert!(store.search_package_name("ackermann_msgs").is_some());
        assert!(store.search_package_name("actionlib").is_some());
        assert!(store.search_package_name("rqt_web").is_some());

        // Doesn't exist
        assert!(store.search_package_name("nope").is_none());

        // No status field
        assert!(store.search_package_name("abseil_cpp").is_none());
    }
}
