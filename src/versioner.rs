use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Versioner {
    versions: HashMap<String, HashSet<String>>,
}

impl Versioner {
    pub fn new() -> Self {
        Self{versions: HashMap::new()}
    }

    pub fn add(&mut self, name: &str, version: &str) {
        let versions = self.versions.entry(name.to_string()).or_insert(HashSet::new());
        versions.insert(version.to_string());
    }
}
