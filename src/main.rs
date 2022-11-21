use crate::{sdk_style_reader::add_packages, versioner::Versioner, packages_reader::add_config_packages};

mod packages_reader;
mod sdk_style_reader;
mod versioner;

fn main() {

    let mut versioner = Versioner::new();
    
    //let path = "./tests/sdk_style_no_package.csproj";
    //let path = "./tests/sdk_style_no_item_group.csproj";
    let path = "./tests/sdk_style.csproj";

    add_packages(path, &mut versioner);

    let path = "./tests/oldstyle/packages.config";
    add_config_packages(path, &mut versioner);

    println!("{:?}", versioner);
}
