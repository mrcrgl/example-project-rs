use std::fmt::Display;

use crate::persistence::config::PersistenceDriver;
use tokio::fs;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub persistence_driver: PersistenceDriver,
}

impl Config {
    pub async fn try_from_file(path: impl Display) -> Result<Self, Box<dyn std::error::Error>> {
        let fh = fs::read(path.to_string()).await?;
        let config = serde_yaml::from_slice(fh.as_slice())?;
        Ok(config)
    }
}
