use common::types::Device;
use common::types::Fleet;
use common::types::Interface;
use common::types::KeyPair;
use common::types::Peer;
use common::types::WireGuard;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_fleet_instantiation() {
        let fleet = Fleet {
            container_registry: "registry.example.com".to_string(),
            created_at: 1234567890,
            description: "Test fleet".to_string(),
            image: "example/image:latest".to_string(),
            name: "TestFleet".to_string(),
            platform: "linux/amd64".to_string(),
            device_uuids: vec!["device-uuid-1".to_string()],
            user_uuids: vec!["user-uuid-1".to_string()],
            uuid: "fleet-uuid".to_string(),
            wireguard_uuids: vec!["wireguard-uuid-1".to_string()],
        };
        assert_eq!(fleet.name, "TestFleet");
    }
    #[test]
    fn test_device_instantiation() {
        let device = Device {
            created_at: 1234567890,
            uuid: "device-uuid".to_string(),
            wireguard_uuid: "wireguard-uuid".to_string(),
            fleet_uuid: "fleet-uuid".to_string(),
            api_url: "https://api.example.com".to_string(),
            file: PathBuf::from("/path/to/device/file"),
        };
        assert_eq!(device.uuid, "device-uuid");
    }
    #[test]
    fn test_wireguard_instantiation() {
        let wireguard = WireGuard {
            created_at: 1234567890,
            uuid: "wireguard-uuid".to_string(),
            device_uuid: "device-uuid".to_string(),
            interface: Interface {
                private_key: "private-key".to_string(),
                address: "10.0.0.1/24".to_string(),
                listen_port: Some(51820),
            },
            peers: vec![Peer {
                public_key: "public-key".to_string(),
                allowed_ips: vec!["10.0.0.2/32".to_string()],
                endpoint: Some("example.com:51820".to_string()),
            }],
            api_url: "https://api.example.com".to_string(),
            file: PathBuf::from("/path/to/wireguard/file"),
            wireguard_file: PathBuf::from("/path/to/wireguard/config"),
        };
        assert_eq!(wireguard.uuid, "wireguard-uuid");
    }
    #[test]
    fn test_interface_instantiation() {
        let interface = Interface {
            private_key: "private-key".to_string(),
            address: "10.0.0.1/24".to_string(),
            listen_port: Some(51820),
        };
        assert_eq!(interface.address, "10.0.0.1/24");
    }
    #[test]
    fn test_peer_instantiation() {
        let peer = Peer {
            public_key: "public-key".to_string(),
            allowed_ips: vec!["10.0.0.2/32".to_string()],
            endpoint: Some("example.com:51820".to_string()),
        };
        assert_eq!(peer.public_key, "public-key");
    }

    #[test]
    fn test_keypair_instantiation() {
        let keypair = KeyPair {
            private_key: "private-key".to_string(),
            public_key: "public-key".to_string(),
        };
        assert_eq!(keypair.private_key, "private-key");
        assert_eq!(keypair.public_key, "public-key");
    }
}
