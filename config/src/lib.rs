use serde::Deserialize;
use std::{
    io::{self},
    path::PathBuf,
};
use toml::from_str;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    pub rpc: RpcConfig,
    pub metrics: MetricsConfig,
    pub network: NetworkConfig,
    pub nostr: NostrConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub write_to_file: bool,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct RpcConfig {
    pub enable_grpc: bool,
    pub grpc_port: u16,
}

#[derive(Debug, Deserialize)]
pub struct MetricsConfig {
    pub enable_metrics: bool,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub port: u16,
    pub bootstraps: Vec<String>,
    pub listen_address: Vec<String>,
    pub max_connections: u16,
    pub name: String,
    pub moniker: String,
}

#[derive(Debug, Deserialize)]
pub struct NostrConfig {
    pub port: u16,
    pub max_ws_connections: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log: LogConfig {
                write_to_file: true,
                path: PathBuf::from("log.r7"),
            },
            rpc: RpcConfig {
                enable_grpc: true,
                grpc_port: 9090,
            },
            metrics: MetricsConfig {
                enable_metrics: false,
            },
            network: NetworkConfig {
                port: 37771,
                bootstraps: Vec::new(),
                listen_address: vec![
                    "/ip4/0.0.0.0/tcp/21777".into(),
                    "/ip4/0.0.0.0/udp/21777/quic-v1".into(),
                    "/ip6/::/tcp/21777".into(),
                    "/ip6/::/udp/21777/quic-v1".into(),
                ],
                max_connections: 10,
                moniker: String::from(""),
                name: String::from("redvin"),
            },
            nostr: NostrConfig {
                port: 443,
                max_ws_connections: 100,
            },
        }
    }
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, io::Error> {
        let contents = std::fs::read_to_string(path)?;
        Ok(from_str(&contents)?)
    }

    pub fn default_file() -> &'static [u8] {
        b"[network]
port = 37771
max_connections = 10
moniker = ''
name = 'redvin'
listen_address = [
    '/ip4/0.0.0.0/tcp/21777',
    '/ip4/0.0.0.0/udp/21777/quic-v1',
	'/ip6/::/tcp/21777',
    '/ip6/::/udp/21777/quic-v1'
]

[nostr]
port = 443
bootstraps = []
max_ws_connections = 100

[rpc]
enable_grpc = true
grpc_port = 9090

[metrics]
enable_metrics = false

[log]
write_to_file = true
path = 'log.r7'
"
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn temp_config_file() {
        let mut p = std::env::temp_dir();
        p.push("./config.toml");
        let mut file = std::fs::File::create(p).expect("Failed to create temp config file");
        file.write_all(Config::default_file())
            .expect("Failed to write to temp config file")
    }

    #[test]
    fn load_from_file_test() {
        temp_config_file();
        let loaded_from_file = Config::load_from_file("./config.toml").unwrap();
        let default = Config::default();

        assert_eq!(loaded_from_file.log.path, default.log.path);
        assert_eq!(
            loaded_from_file.log.write_to_file,
            default.log.write_to_file
        );

        assert_eq!(
            loaded_from_file.metrics.enable_metrics,
            default.metrics.enable_metrics
        );

        assert_eq!(
            loaded_from_file.network.max_connections,
            default.network.max_connections
        );
        assert_eq!(loaded_from_file.network.port, default.network.port);
        assert_eq!(
            loaded_from_file.network.bootstraps,
            default.network.bootstraps
        );
        assert_eq!(loaded_from_file.network.moniker, default.network.moniker);
        assert_eq!(loaded_from_file.network.name, default.network.name);
        assert_eq!(
            loaded_from_file.network.listen_address,
            default.network.listen_address
        );

        assert_eq!(
            loaded_from_file.nostr.max_ws_connections,
            default.nostr.max_ws_connections
        );
        assert_eq!(loaded_from_file.nostr.port, default.nostr.port);

        assert_eq!(loaded_from_file.rpc.enable_grpc, default.rpc.enable_grpc);
        assert_eq!(loaded_from_file.rpc.grpc_port, default.rpc.grpc_port);
    }

    #[test]
    fn default_test() {
        let default = Config::default();
        let bootstraps: Vec<String> = Vec::new();

        assert_eq!(PathBuf::from("log.r7"), default.log.path);
        assert_eq!(true, default.log.write_to_file);

        assert_eq!(false, default.metrics.enable_metrics);

        assert_eq!(10, default.network.max_connections);
        assert_eq!(37771, default.network.port);
        assert_eq!(bootstraps, default.network.bootstraps);
        assert_eq!(String::from(""), default.network.moniker);
        assert_eq!(
            vec![
                "/ip4/0.0.0.0/tcp/21777",
                "/ip4/0.0.0.0/udp/21777/quic-v1".into(),
                "/ip6/::/tcp/21777".into(),
                "/ip6/::/udp/21777/quic-v1".into(),
            ],
            default.network.listen_address
        );
        assert_eq!(String::from("redvin"), default.network.name);

        assert_eq!(100, default.nostr.max_ws_connections);
        assert_eq!(443, default.nostr.port);

        assert_eq!(true, default.rpc.enable_grpc);
        assert_eq!(9090, default.rpc.grpc_port);
    }
}
