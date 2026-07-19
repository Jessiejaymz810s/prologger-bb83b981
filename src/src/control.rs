// src/config.rs
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DappConfig {
    pub app_name: String,
    pub environment: String,
    
    // We use a custom remote attribute to handle deserializing log::LevelFilter seamlessly
    #[serde(with = "LevelFilterDef")]
    pub default_level: LevelFilter,
    
    pub enable_json_format: bool,
    pub target_endpoint: Option<String>,
}

// Helper enum mapper to tell Serde how to interpret the foreign log::LevelFilter type
#[derive(Serialize, Deserialize)]
#[serde(remote = "LevelFilter", rename_all = "lowercase")]
enum LevelFilterDef {
    Off, Error, Warn, Info, Debug, Trace,
}

impl DappConfig {
    /// Loads the configuration out of a local JSON target file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
