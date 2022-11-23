use std::{fs::File, io::BufReader, path::Path};

use quick_xml::de::from_reader;
use serde::Deserialize;

use crate::versioner::Versioner;

#[derive(Deserialize, Debug)]
struct Package {
    id: String,
    version: String,
}

#[derive(Deserialize, Debug)]
struct Packages {
    package: Vec<Package>,
}

fn read_package<P: AsRef<Path>>(path: P) -> Packages {
    let f = File::open(path).expect("package read");
    let b = BufReader::new(f);

    from_reader(b).expect("packages not read")
}

pub fn add_config_packages<P: AsRef<Path>>(path: P, versioner: &mut Versioner) {
    let packages = read_package(path);

    for package in packages.package {
        versioner.add(&package.id, &package.version);
    }
}

#[cfg(test)]
mod tests {
    use crate::packages_reader::read_package;

    #[test]
    fn parse_packages() {
        let result = read_package("./tests/oldstyle/packages.config");
        assert_eq!(result.package.len(), 6);
        assert_eq!(result.package[0].version, "3.1.0");
    }
}
