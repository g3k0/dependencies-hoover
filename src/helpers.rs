use std::path::{Path, PathBuf};
use std::{fs};
use serde_json::{Value, from_str};
use std::collections::HashMap;

extern crate serde;
extern crate serde_derive;

pub fn scan_directory(path: &Path, ignore_dirs: &Vec<String>) {
    if path.is_dir() && !is_dir_in_ignore_list(&path, &ignore_dirs) {

        let Ok(_dir_entries): Result<fs::ReadDir, std::io::Error> = fs::read_dir(path) else { return };

        for entry in fs::read_dir(path).unwrap() {
            let entry_path: std::path::PathBuf = entry.unwrap().path();
            scan_directory(&entry_path, &ignore_dirs);
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
                if !is_dependency_used(&dep, &path.with_file_name(""), ignore_dirs) && 
                   !file_exists_in_node_modules_bin(&dep, &path) {
                    // delete_dependency(path, dependency);
                    println!("{} | {}", path.display(), dep)
                }
            }
        }
    }
}


fn is_dependency_used(dependency: &str, path: &Path, ignore_dirs: &Vec<String>) -> bool {
    let mut usage = false;
    if path.is_dir() && !is_dir_in_ignore_list(&path, &ignore_dirs) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            if is_dependency_used(dependency, &entry_path, ignore_dirs) {
                usage = true;
                break;
            }
        }
    } else if !path.extension().is_none() && (path.extension().unwrap() == "js" || path.extension().unwrap() == "ts") {
        let file_contents = fs::read_to_string(path).unwrap();
        if file_contents.contains(&format!("import \"{}", dependency)) ||
            file_contents.contains(&format!("import '{}", dependency)) ||
            file_contents.contains(&format!("import {}", dependency)) ||
            file_contents.contains(&format!("require(\"{}", dependency)) ||
            file_contents.contains(&format!("require('{}", dependency)) || 
            file_contents.contains(&format!("from '{}", dependency)) ||  
            file_contents.contains(&format!("from \"{}", dependency)) {
            usage = true
        }
    }

    usage
}

fn file_exists_in_node_modules_bin(dependency: &str, path: &Path) -> bool {
    let file_path: PathBuf = path.with_file_name("node_modules/.bin").join(dependency);
    file_path.is_file()
}

fn is_dir_in_ignore_list(path: &Path, ignore: &Vec<String>) -> bool {
     if !path.is_dir() {
        return false;
    }
    let path_str = path.to_str().unwrap();
    for dir in ignore {
        if path_str.contains(dir) {
            return true;
        }
    }
    false
}

/* fn delete_dependency(package_json_path: &Path, dependency: &str) {
    let file_contents = fs::read_to_string(package_json_path).unwrap();
    let mut package_json: PackageJson = serde_json::from_str(&file_contents).unwrap();

    if package_json.dependencies.remove(dependency).is_some() {
        println!("Dependency {} removed from dependencies.", dependency);
    } else if package_json.devDependencies.remove(dependency).is_some() {
        println!("Dependency {} removed from devDependencies.", dependency);
    } else {
        println!("Dependency {} not found in package.json.", dependency);
    }

    let updated_package_json = serde_json::to_string(&package_json).unwrap();
    fs::write(package_json_path, updated_package_json).unwrap();
} */
