use std::convert::From;
use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::config::{parse_to_config_file, AppConfig, TomlConfigFile};

use super::app::{AppBatteryConfig, AppCpuConfig};

const fn default_poll_rate() -> u64 {
    5000
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AppCpuConfigRaw {
    #[serde(default)]
    pub temperature_file: Option<PathBuf>,
    #[serde(default)]
    pub temperature_denominator: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AppBatteryConfigRaw {
    #[serde(default)]
    pub charge_file: Option<PathBuf>,
    #[serde(default)]
    pub status_file: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigRaw {
    #[serde(default = "default_poll_rate")]
    pub poll_rate: u64,
    #[serde(default)]
    pub cpu: AppCpuConfigRaw,
    pub battery: AppBatteryConfigRaw,
}

impl From<AppConfigRaw> for AppConfig {
    fn from(raw: AppConfigRaw) -> Self {
        Self {
            poll_rate: raw.poll_rate,
            cpu: AppCpuConfig::from(raw.cpu),
            battery: AppBatteryConfig::from(raw.battery),
        }
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppConfigRaw, AppConfig>(file_name).unwrap_or_default()
    }
}
