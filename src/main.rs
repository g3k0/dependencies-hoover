use std::fs;
use std::path::Path;

extern crate serde;
#[macro_use]
extern crate serde_derive;

fn main() {
    let root_path = Path::new("/Users/palazzoc0905/Studio/oao-webplex/");
    scan_directory(&root_path);
}

fn scan_directory(path: &Path) {
    if path.is_dir() {

        let _dir_entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(_) => return,
        };

        for entry in fs::read_dir(path).unwrap() {

            let entry_path = if let Ok(ref entry) = entry {
                entry.path()
            } else {
                continue
            };

            let file_name = entry_path.file_name().unwrap().to_str().unwrap();
            if file_name == "node_modules" {
                continue;
            }

            let entry_path = entry.unwrap().path();
            scan_directory(&entry_path);
        }
    } else if path.file_name().unwrap() == "package.json" {
        let file_contents = fs::read_to_string(path).unwrap();
        let package_json: PackageJson = serde_json::from_str(&file_contents).unwrap();
        let dependencies = package_json.dependencies.keys().collect::<Vec<_>>();
        let dev_dependencies = package_json.devDependencies.keys().collect::<Vec<_>>();
        let all_dependencies = [dependencies, dev_dependencies].concat();
        for dependency in all_dependencies {
            if !is_dependency_used(dependency, path) && !file_exists_in_node_modules_bin(dependency, path) {
                // delete_dependency(path, dependency);
                println!("{} | {}", path.display(), dependency)
            }
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
struct PackageJson {
    private: bool,
    name: String,
    version: String,
    description: String,
    license: String,
    author: String,
    scripts: std::collections::HashMap<String, String>,
    config: std::collections::HashMap<String, String>,
    workspaces: std::collections::HashMap<String, String>,
    resolutions: std::collections::HashMap<String, String>,
    dependencies: std::collections::HashMap<String, String>,
    optionalDependencies: std::collections::HashMap<String, String>,
    devDependencies: std::collections::HashMap<String, String>,
    engines: std::collections::HashMap<String, String>,
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

fn is_dependency_used(dependency: &str, path: &Path) -> bool {
    let mut is_used = false;

    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry_path = entry.unwrap().path();
            if is_dependency_used(dependency, &entry_path) {
                is_used = true;
                break;
            }
        }
    } else if path.extension().unwrap() == "js" || path.extension().unwrap() == "ts" {
        let file_contents = fs::read_to_string(path).unwrap();
        if file_contents.contains(&format!("use {}", dependency)) {
            is_used = true;
        }
    }

    is_used
}

fn file_exists_in_node_modules_bin(dependency: &str, path: &Path) -> bool {
    let file_path = path.join("node_modules/.bin").join(dependency);
    file_path.is_file()
}
