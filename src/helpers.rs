use std::path::{Path, PathBuf};
use std::{fs};

pub fn is_dependency_used(dependency: &str, path: &Path) -> bool {
    let mut usage = false;
    if path.is_dir() {
        let dir_name = path.file_name().unwrap().to_str().unwrap();
        if dir_name != "node_modules" {
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                if is_dependency_used(dependency, &entry_path) {
                    usage = true;
                    break;
                }
            }
        }
    } else if !path.extension().is_none() && (path.extension().unwrap() == "js" || path.extension().unwrap() == "ts") {
        let file_contents = fs::read_to_string(path).unwrap();
        if file_contents.contains(&format!("import \"{}", dependency)) ||
            file_contents.contains(&format!("import '{}", dependency)) ||
            file_contents.contains(&format!("import {}", dependency)) ||
            file_contents.contains(&format!("require(\"{}", dependency)) ||
            file_contents.contains(&format!("require('{}", dependency)) {
            usage = true
        }
    }

    usage
}

pub fn file_exists_in_node_modules_bin(dependency: &str, path: &Path) -> bool {
    let file_path: PathBuf = path.with_file_name("node_modules/.bin").join(dependency);
    file_path.is_file()
}

/* pub fn delete_dependency(package_json_path: &Path, dependency: &str) {
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
