use std::convert::From;
use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::config::{parse_to_config_file, AppConfig, TomlConfigFile};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigRaw {
    #[serde(default)]
    pub cpu_temp_file: Option<PathBuf>,
    #[serde(default)]
    pub cpu_temp_denominator: Option<f32>,

    #[serde(default)]
    pub ac_file: Option<PathBuf>,

    #[serde(default)]
    pub battery_file: Option<PathBuf>,
    #[serde(default)]
    pub battery_denominator: Option<f32>,
}

impl From<AppConfigRaw> for AppConfig {
    fn from(raw: AppConfigRaw) -> Self {
        Self {
            cpu_temp_file: raw.cpu_temp_file,
            cpu_temp_denominator: raw.cpu_temp_denominator,

            ac_file: raw.ac_file,

            battery_file: raw.battery_file,
            battery_denominator: raw.battery_denominator,
        }
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppConfigRaw, AppConfig>(file_name).unwrap_or_default()
    }
}
