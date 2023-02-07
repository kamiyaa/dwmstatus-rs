use std::io;
use std::path::PathBuf;

use super::DEFAULT_CONFIG_FILE_PATH;

use super::app_raw::{AppBatteryConfigRaw, AppConfigRaw, AppCpuConfigRaw};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub cpu: AppCpuConfig,
    pub battery: AppBatteryConfig,
}

impl AppConfig {
    pub fn default_res() -> io::Result<Self> {
        let raw: Result<AppConfigRaw, toml::de::Error> = toml::from_str(DEFAULT_CONFIG_FILE_PATH);
        match raw {
            Ok(raw) => Ok(Self::from(raw)),
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "")),
        }
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::default_res().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct AppCpuConfig {
    pub temperature: Option<PathBuf>,
    pub temperature_denominator: Option<f32>,
}

impl From<AppCpuConfigRaw> for AppCpuConfig {
    fn from(config: AppCpuConfigRaw) -> Self {
        Self {
            temperature: config.temperature,
            temperature_denominator: config.temperature_denominator,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppBatteryConfig {
    pub percentage: Option<PathBuf>,
    pub status: Option<PathBuf>,
}

impl From<AppBatteryConfigRaw> for AppBatteryConfig {
    fn from(config: AppBatteryConfigRaw) -> Self {
        Self {
            percentage: config.percentage,
            status: config.status,
        }
    }
}
