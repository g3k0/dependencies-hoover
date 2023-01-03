use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::{fs, io};
use chrono::{DateTime, Local};
use serde_json::{Value, from_str};
use std::collections::HashMap;
use std::io::prelude::*;

extern crate serde;
extern crate serde_derive;
extern crate regex;

use regex::Regex;

pub fn scan_directory(path: &Path, ignore_dirs: &Vec<String>, dependencies_whitelist: &Vec<String>) {
    if path.is_dir() && !is_dir_in_ignore_list(&path, &ignore_dirs) {

        let Ok(_dir_entries): Result<fs::ReadDir, std::io::Error> = fs::read_dir(path) else { return };

        for entry in fs::read_dir(path).unwrap() {
            let entry_path: std::path::PathBuf = entry.unwrap().path();
            scan_directory(&entry_path, &ignore_dirs, &dependencies_whitelist);
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
                if  !file_exists_in_node_modules_bin(&dep, &path) &&
                    !is_dependency_in_whitelist(&dep, &dependencies_whitelist) {
                    if !is_dependency_used(&dep, &path.with_file_name(""), ignore_dirs) {

                        delete_dependency(path, dep).unwrap();
                        match write_report(path, dep) {
                            Ok(()) => (),
                            Err(e) => panic!("Error: {}", e),
                        }

                    }
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
    } else if !path.extension().is_none() && 
              (path.extension().unwrap() == "js" || path.extension().unwrap() == "ts") {
        let file_contents = fs::read_to_string(path).unwrap();
        if file_contents.contains(&format!("import \"{}", dependency)) ||
            file_contents.contains(&format!("import '{}", dependency)) ||
            file_contents.contains(&format!("import {}", dependency)) ||
            file_contents.contains(&format!("(\"{}", dependency)) ||
            file_contents.contains(&format!("('{}", dependency)) || 
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

fn is_dependency_in_whitelist(dependency: &str, whitelist: &Vec<String>) -> bool {
    for pattern in whitelist {
        let regex = Regex::new(&pattern).unwrap();
        if regex.is_match(dependency) {
            return true;
        }
    }
    false
}

fn delete_dependency(package_json_path: &Path, dependency: &str) -> io::Result<()> {
    let mut file = fs::File::open(package_json_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut json: serde_json::Value = serde_json::from_str(&contents)?;

    if let Some(_deps) = json.get("dependencies") {
        if let Some(deps) = json["dependencies"].as_object_mut() {
            deps.remove(dependency);
        }
    }

    if let Some(_deps) = json.get("devDependencies") {
        if let Some(deps) = json["devDependencies"].as_object_mut() {
            deps.remove(dependency);
        }
    }   

    let new_contents = serde_json::to_string_pretty(&json)?;
    let mut file = fs::File::create(package_json_path)?;
    file.write_all(new_contents.as_bytes())?;

    Ok(())
}

fn write_report(path: &Path, dependency: &str) -> std::io::Result<()> {
    let today:DateTime<Local> = Local::now();
    let filename = "./reports/dependencies_cleaning_report_".to_owned() + &today.format("%Y-%m-%d").to_string();
    let report_path: &Path = Path::new(&filename);
    let line: String = path.display().to_string() + " | " + dependency;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(report_path)?;

    file.write_all(line.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

// ----------------------------------------------------------------------------------------------------------------- //
/**
 *  UNIT TESTS
 */

#[cfg(test)]

#[test]
fn test_is_dependency_used() {
    let dependency = "lodash";
    let ignore_dirs = vec!["node_modules".to_string()];

    // Test directory with no usage
    let path = Path::new("./mock/lib");
    assert_eq!(is_dependency_used(dependency, &path, &ignore_dirs), false);

    // Test directory with usage in a file
    let path = Path::new("./mock/app");
    assert_eq!(is_dependency_used(dependency, &path, &ignore_dirs), true);

    // Test file with usage
    let path = Path::new("./mock/app/index.js");
    assert_eq!(is_dependency_used(dependency, &path, &ignore_dirs), true);

    // Test file without usage
    let path = Path::new("./mock/lib/utils.js");
    assert_eq!(is_dependency_used(dependency, &path, &ignore_dirs), false);
}

#[test]
fn test_file_exists_in_node_modules_bin() {
    let dependency_1: &str = "nx";
    let dependency_2: &str = "express";
    let path: &Path = Path::new("./mock/app/package.json");

    // Test dependency with no bin
    assert_eq!(file_exists_in_node_modules_bin(dependency_2, &path), false);

    // Test dependency with bin
    assert_eq!(file_exists_in_node_modules_bin(dependency_1, &path), true);
}

#[test]
fn test_is_dir_in_ignore_list() {
    let ignore_dirs = vec!["node_modules".to_string(), "build".to_string()];

    // Test directory in ignore list
    let path = Path::new("./mock/app/node_modules");
    assert_eq!(is_dir_in_ignore_list(&path, &ignore_dirs), true);

    // Test directory not in ignore list
    let path = Path::new("./mock/app/src");
    assert_eq!(is_dir_in_ignore_list(&path, &ignore_dirs), false);

    // Test file instead of directory
    let path = Path::new("./mock/app/src/main.js");
    assert_eq!(is_dir_in_ignore_list(&path, &ignore_dirs), false);
}

#[test]
fn test_is_dependency_in_whitelist() {
    let whitelist = vec![
        "^@types/.*$".to_string(),
        "^react-router-dom$".to_string(),
    ];

    // Test dependency in whitelist
    let dependency = "@types/react-router-dom";
    assert_eq!(is_dependency_in_whitelist(dependency, &whitelist), true);

    // Test dependency not in whitelist
    let dependency = "lodash";
    assert_eq!(is_dependency_in_whitelist(dependency, &whitelist), false);
}

#[test]
fn test_write_report() {
    let path = Path::new("/app/src/main.js");
    let dependency = "lodash";

    // Write report
    write_report(path, dependency).expect("Failed to write report");

    // Check if report was written correctly
    let today:DateTime<Local> = Local::now();
    let filename = "./reports/dependencies_cleaning_report_".to_owned() + &today.format("%Y-%m-%d").to_string();
    let report_path = Path::new(&filename);
    let report_contents = fs::read_to_string(report_path).expect("Failed to read report");
    assert!(report_contents.contains(&path.display().to_string()));
    assert!(report_contents.contains(dependency));

    // Delete report
    fs::remove_file(report_path).expect("Failed to delete report");
}
