use std::{fs};
use std::path::{Path};
use serde_json::{Value, from_str};
use std::collections::HashMap;
use config::Config;

mod helpers;

extern crate serde;
extern crate serde_derive;

fn main() {

    // Read config file

    let settings_config: Config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap();

    let settings:HashMap<String, String> = settings_config
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();
    let project_path: &str = settings.get("projectToScanPath").unwrap();

    let root_path: &Path = Path::new(project_path);
    scan_directory(&root_path);
}

fn scan_directory(path: &Path) {
    if path.is_dir() && !path.ends_with("node_modules") {

        let Ok(_dir_entries): Result<fs::ReadDir, std::io::Error> = fs::read_dir(path) else { return };

        for entry in fs::read_dir(path).unwrap() {

            let entry_path: std::path::PathBuf = if let Ok(ref entry) = entry {
                entry.path()
            } else {
                continue
            };

            let file_name: &str = entry_path.file_name().unwrap().to_str().unwrap();
            if file_name == "node_modules" {
                continue;
            }

            let entry_path: std::path::PathBuf = entry.unwrap().path();
            scan_directory(&entry_path);
        }
    } else if path.file_name().unwrap() == "package.json" {
        let file_contents: String = fs::read_to_string(path).unwrap();
        let package_json: Value = from_str(&file_contents).unwrap();

        if !package_json["dependencies"].is_null() {
            let mut dependencies_map: HashMap<String, Value> = package_json["dependencies"].as_object().unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

            if !package_json["devDependencies"].is_null() {
                let dev_dependencies_map: HashMap<String, Value> = package_json["devDependencies"].as_object().unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

                dependencies_map.extend(dev_dependencies_map);
            }

            for dependency in &dependencies_map {
                let dep: &str = dependency.0.as_str();
                if !helpers::is_dependency_used(&dep, &path.with_file_name("")) && 
                   !helpers::file_exists_in_node_modules_bin(&dep, &path) {
                    // delete_dependency(path, dependency);
                    println!("{} | {}", path.display(), dep)
                }
            }
        }
    }
}
