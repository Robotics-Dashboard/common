use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fleet {
    pub container_registry: String,
    pub created_at: u64,
    pub description: String,
    pub image: String,
    pub name: String,
    pub platform: String,
    pub device_uuids: Vec<String>,
    pub user_uuids: Vec<String>,
    pub uuid: String,
    pub wireguard_uuids: Vec<String>,
}

impl fmt::Display for Fleet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}, platform: {}, image: {}, created_at: {}, description: {}, container_registry: {}, uuid: {}, devices: [{}], users: [{}], wireguards: [{}]",
            self.name,
            self.platform,
            self.image,
            self.created_at,
            self.description,
            self.container_registry,
            self.uuid,
            self.device_uuids.join(", "),
            self.user_uuids.join(", "),
            self.wireguard_uuids.join(", ")
        )
    }
}
