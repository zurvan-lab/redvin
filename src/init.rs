use config::Config;
use std::fs;

pub fn init(working_directory: &str) {
    let dir_builder = fs::DirBuilder::new();

    dir_builder
        .create(working_directory)
        .expect("can not create working directory.");

    let config_path = format!("{}/config.toml", working_directory);
    fs::write(config_path, Config::default_file())
        .expect("can not create config file on working directory.");
}
