mod config;
mod run;

use std::path::PathBuf;

use lazy_static::lazy_static;

use crate::config::{AppConfig, TomlConfigFile};

const PROGRAM_NAME: &str = "dwmstatus";
const CONFIG_HOME: &str = "DWMSTATUS_CONFIG_HOME";
const CONFIG_FILE: &str = "config.toml";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut config_dirs = vec![];

        if let Ok(p) = std::env::var(CONFIG_HOME) {
            let p = PathBuf::from(p);
            if p.is_dir() {
                config_dirs.push(p);
            }
        }

        if let Ok(p) = std::env::var("HOME") {
            let mut p = PathBuf::from(p);
            p.push(format!(".config/{}", PROGRAM_NAME));
            if p.is_dir() {
                config_dirs.push(p);
            }
        }
        config_dirs
    };
}

fn main() {
    let config = AppConfig::get_config(CONFIG_FILE);

    run::run(config);
}
