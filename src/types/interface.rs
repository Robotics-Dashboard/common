use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interface {
    pub private_key: String,
    pub address: String,
    pub listen_port: Option<u16>,
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[Interface]")?;
        writeln!(f, "PrivateKey = {}", self.private_key)?;
        writeln!(f, "Address = {}", self.address)?;
        if let Some(port) = self.listen_port {
            writeln!(f, "ListenPort = {}", port)?;
        }
        Ok(())
    }
}
