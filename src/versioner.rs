use std::collections::{HashSet, BTreeMap};

#[derive(Debug)]
pub struct Versioner {
    package_versions: BTreeMap<String, HashSet<String>>,
}

impl Versioner {
    pub fn new() -> Self {
        Self{package_versions: BTreeMap::new()}
    }

    pub fn add(&mut self, name: &str, version: &str) {
        let versions = self.package_versions.entry(name.to_string()).or_insert_with(HashSet::new);
        versions.insert(version.to_string());
    }

    pub fn print_multiple(&self) {
        println!("--------------------");
        let mut something_found = false;
        for (name, versions) in self.package_versions.iter() {
            if versions.len() > 1 {
                something_found = true;
                print!("MULTIPLE for {}: ", name);
                for version in versions {
                    print!("{}, ", version);
                }
                println!();
            }
        }

        if !something_found {
            println!("nothing found");
        }
        println!("--------------------");
    }
}
