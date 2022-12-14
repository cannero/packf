use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;
use quick_xml::de::from_reader;

use crate::versioner::Versioner;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct PackageReference {
    include: String,
    version: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ItemGroup {
    #[serde(default)]
    package_reference: Vec<PackageReference>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Project {
    #[serde(default)]
    item_group: Vec<ItemGroup>,
}

fn read_csproj<P: AsRef<Path>>(path: P) -> Project {
    let f = File::open(path).expect("csproj read");
    let b = BufReader::new(f);

    from_reader(b).expect("check if ItemGroup nodes are in sequence")
}

pub fn add_packages<P: AsRef<Path>>(path: P, versioner: &mut Versioner){
    let project = read_csproj(path);
    for item_group in project.item_group {
        for package in item_group.package_reference {
            versioner.add(&package.include, &package.version);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sdk_style_reader::read_csproj;

    #[test]
    fn parse_new_style() {
        let result = read_csproj("./tests/sdk_style.csproj");
        let item_groups = result.item_group;
        assert_eq!(item_groups.len(), 3);
        let package_ref: &Vec<_> = &item_groups[0].package_reference;
        assert_eq!(package_ref.len(), 5);
    }

    #[test]
    fn parse_no_packages() {
        let result = read_csproj("./tests/sdk_style_no_package.csproj");
        let item_groups = result.item_group;
        assert_eq!(item_groups.len(), 2);
        assert!(item_groups[0].package_reference.is_empty());
        assert!(item_groups[1].package_reference.is_empty());
    }

    #[test]
    fn parse_no_item_groups() {
        let result = read_csproj("./tests/sdk_style_no_item_group.csproj");
        assert!(result.item_group.is_empty());
    }
}
