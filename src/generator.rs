use std::path::PathBuf;
use walkdir::WalkDir;
use crate::config::Config;

pub fn generate(config_file: &PathBuf, _config: &Config) {
    let main_dir = config_file.parent().unwrap();

    for file in WalkDir::new(main_dir).into_iter().filter_map(|file| file.ok()) {
        //if file.metadata().unwrap().is_file() {
            println!("{:?}", file);
        //}
    }
}