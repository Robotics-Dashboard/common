use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Peer {
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub endpoint: Option<String>,
}

impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Peer]")?;
        writeln!(f, "PublicKey = {}", self.public_key)?;
        writeln!(f, "AllowedIPs = {}", self.allowed_ips.join(", "))?;
        if let Some(endpoint) = &self.endpoint {
            writeln!(f, "Endpoint = {}", endpoint)?;
        }
        Ok(())
    }
}
