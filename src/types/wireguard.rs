use crate::errors::NotSetError;
use crate::traits::{Fetchable, Parsable, Storeable, Validatable};
use crate::types::interface::Interface;
use crate::types::peer::Peer;

use anyhow::{Context, Result};
use async_trait::async_trait;
use ini::Ini;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WireGuard {
    pub created_at: u64,
    pub uuid: String,
    pub device_uuid: String,
    pub interface: Interface,
    pub peers: Vec<Peer>,
    pub api_url: String,
    pub file: PathBuf,
    pub wireguard_file: PathBuf,
}

#[async_trait]
impl Fetchable for WireGuard {
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

impl Storeable for WireGuard {
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

impl Validatable for WireGuard {
    fn validate(&self) -> Result<&Self, NotSetError> {
        if self.created_at == 0 {
            return Err(NotSetError::CreatedAt);
        }
        if self.interface.private_key.is_empty() {
            return Err(NotSetError::PrivateKey);
        }
        if self.interface.address.is_empty() {
            return Err(NotSetError::Address);
        }
        if self.peers.is_empty() {
            return Err(NotSetError::Peers);
        }
        if self.api_url.is_empty() {
            return Err(NotSetError::ApiUrl);
        }
        if self.file.as_os_str().is_empty() {
            return Err(NotSetError::File);
        }
        if self.wireguard_file.as_os_str().is_empty() {
            return Err(NotSetError::WireguardFile);
        }
        Ok(self)
    }
}

impl Parsable for WireGuard {
    fn parse(&self, path: &Path) -> Result<Self> {
        let conf = Ini::load_from_file(path)?;
        let mut interface = Interface {
            private_key: String::new(),
            address: String::new(),
            listen_port: None,
        };
        let mut peers = Vec::new();
        for (sec, prop) in conf.iter() {
            match sec {
                Some(section) if section.starts_with("Interface") => {
                    for (key, value) in prop.iter() {
                        match key {
                            "PrivateKey" => interface.private_key = value.into(),
                            "Address" => interface.address = value.into(),
                            "ListenPort" => interface.listen_port = value.parse().ok(),
                            // Handle other fields
                            _ => {}
                        }
                    }
                }
                Some(section) if section.starts_with("Peer") => {
                    let mut peer = Peer {
                        public_key: String::new(),
                        allowed_ips: Vec::new(),
                        endpoint: None,
                    };
                    for (key, value) in prop.iter() {
                        match key {
                            "PublicKey" => peer.public_key = value.into(),
                            "AllowedIPs" => {
                                peer.allowed_ips = value.split(',').map(String::from).collect()
                            }
                            "Endpoint" => peer.endpoint = Some(value.into()),
                            // Handle other fields
                            _ => {}
                        }
                    }
                    peers.push(peer);
                }
                _ => {}
            }
        }
        Ok(WireGuard {
            interface,
            peers,
            ..self.clone()
        })
    }
}

impl fmt::Display for WireGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.interface)?;
        for peer in &self.peers {
            writeln!(f)?;
            write!(f, "{}", peer)?;
        }
        Ok(())
    }
}
