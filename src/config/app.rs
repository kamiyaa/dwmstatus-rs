use std::io;
use std::path::PathBuf;

use super::DEFAULT_CONFIG_FILE_PATH;

use super::app_raw::AppConfigRaw;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub cpu_temp_file: Option<PathBuf>,
    pub cpu_temp_denominator: Option<f32>,

    pub battery_file: Option<PathBuf>,
    pub battery_drain: Option<PathBuf>,
}

impl AppConfig {
    pub fn default_res() -> io::Result<Self> {
        let raw: AppConfigRaw = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(raw))
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::default_res().unwrap()
    }
}
