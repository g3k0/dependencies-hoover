use std::collections::HashMap;
use config::Config;
use std::path::{Path};

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
    helpers::scan_directory(&root_path);
}
