use config::Config;
use serde::Deserialize;
use std::path::{Path};

mod helpers;

extern crate serde;
extern crate serde_derive;

#[derive(Deserialize)]
struct Settings {
    project_to_scan_path: String,
    analysis_only: bool,
    ignore_dirs: Vec<String>,
    dependencies_whitelist: Vec<String>,
}

fn main() {
    // Read config file
    let settings_config: Config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .unwrap(); 
    let settings: Settings = settings_config.try_deserialize().unwrap();

    // get the config properties
    let project_path: &str = &settings.project_to_scan_path;
    let analysis_only: &bool = &settings.analysis_only;
    let root_path: &Path = Path::new(project_path);
    let ignore_dirs: Vec<String> = settings.ignore_dirs;
    let dependencies_whitelist: Vec<String> = settings.dependencies_whitelist;

    // scan the project
    helpers::scan_directory(
        &root_path, 
        &analysis_only,
        &ignore_dirs, 
        &dependencies_whitelist
    );
}
