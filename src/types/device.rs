use crate::errors::NotSetError;
use crate::traits::{Fetchable, Storeable, Validatable};

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub created_at: u64,
    pub uuid: String,
    pub wireguard_uuid: String,
    pub fleet_uuid: String,
    pub api_url: String,
    pub file: PathBuf,
}

#[async_trait]
impl Fetchable for Device {
    async fn fetch(&self) -> Result<Self>
    where
        Self: Sized,
    {
        let response = reqwest::Client::new()
            .post(self.api_url.clone())
            .json(&self)
            .send()
            .await?;
        let config: Self = response.json().await?;
        Ok(config)
    }
}

impl Storeable for Device {
    fn load(file_path: &Path) -> Result<Self>
    where
        Self: Sized,
    {
        let mut file = File::open(file_path)
            .with_context(|| format!("Unable to open config file at {:?}", file_path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Unable to read config file")?;
        let config: Self = serde_yaml::from_str(&contents).context("Unable to deserialize YAML")?;
        Ok(config)
    }
    fn save(&self) -> Result<()> {
        let file_path = self.file.clone();
        let mut file = File::create(file_path)?;
        let yaml = serde_yaml::to_string(&self)?;
        file.write_all(yaml.as_bytes())?;
        Ok(())
    }
}

impl Validatable for Device {
    fn validate(&self) -> Result<&Self, NotSetError> {
        if self.created_at == 0 {
            return Err(NotSetError::CreatedAt);
        }
        if self.uuid.is_empty() {
            return Err(NotSetError::Uuid);
        }
        if self.fleet_uuid.is_empty() {
            return Err(NotSetError::Fleet);
        }
        if self.api_url.is_empty() {
            return Err(NotSetError::ApiUrl);
        }
        if self.file.as_os_str().is_empty() {
            return Err(NotSetError::File);
        }
        Ok(self)
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "uuid: {}, fleet: {}, api_url: {}",
            self.uuid, self.fleet_uuid, self.api_url
        )
    }
}
