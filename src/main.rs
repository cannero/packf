use std::{ffi::OsStr, env};

use walkdir::WalkDir;

use crate::{sdk_style_reader::add_packages, versioner::Versioner, packages_reader::add_config_packages};

mod packages_reader;
mod sdk_style_reader;
mod versioner;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("missing path");
        return;
    }

    println!("searching {}", &args[1]);

    let mut versioner = Versioner::new();

    for entry in WalkDir::new(&args[1])
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {
            let path = entry.path();
            if path.extension() == Some(OsStr::new("csproj")) {
                println!("found {:?}", path);
                add_packages(path, &mut versioner);
            } else if entry.file_name() == "packages.config" {
                println!("pack {:?}", path);
                add_config_packages(path, &mut versioner);
            }
        }

    println!("{:?}", versioner);
}
