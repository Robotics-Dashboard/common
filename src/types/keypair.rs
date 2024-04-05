use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyPair {
    pub private_key: String,
    pub public_key: String,
}
