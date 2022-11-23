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
    package_reference: Option<Vec<PackageReference>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Project {
    item_group: Option<Vec<ItemGroup>>,
}

fn read_csproj<P: AsRef<Path>>(path: P) -> Project {
    let f = File::open(path).expect("csproj read");
    let b = BufReader::new(f);

    from_reader(b).expect("check nodes without end")
}

pub fn add_packages<P: AsRef<Path>>(path: P, versioner: &mut Versioner){
    let project = read_csproj(path);
    if let Some(item_groups) = project.item_group {
        for item_group in item_groups {
            if let Some(packages) = item_group.package_reference {
                for package in packages {
                    versioner.add(&package.include, &package.version);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sdk_style_reader::read_csproj;

    #[test]
    fn parse_new_style() {
        let result = read_csproj("./tests/sdk_style.csproj");
        let item_groups = result.item_group.unwrap();
        assert_eq!(item_groups.len(), 3);
        let package_ref = item_groups[0].package_reference.as_ref().unwrap();
        assert_eq!(package_ref.len(), 5);
    }

    #[test]
    fn parse_no_packages() {
        let result = read_csproj("./tests/sdk_style_no_package.csproj");
        let item_groups = result.item_group.unwrap();
        assert_eq!(item_groups.len(), 2);
        assert!(item_groups[0].package_reference.is_none());
        assert!(item_groups[1].package_reference.is_none());
    }

    #[test]
    fn parse_no_item_groups() {
        let result = read_csproj("./tests/sdk_style_no_item_group.csproj");
        assert!(result.item_group.is_none());
    }
}
