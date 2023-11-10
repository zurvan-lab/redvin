use serde::Deserialize;
use std::error::Error;
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
    pub path: String,
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
    pub max_connections: u16,
}

#[derive(Debug, Deserialize)]
pub struct NostrConfig {
    pub port: u16,
    pub max_ws_connections: u32,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = from_str(&contents)?;
        Ok(config)
    }

    pub fn default() -> Result<Self, Box<dyn Error>> {
        let config: Config = Config {
            log: LogConfig {
                write_to_file: true,
                path: "log.r7".to_owned(),
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
                max_connections: 10,
            },
            nostr: NostrConfig {
                port: 4444,
                max_ws_connections: 1000000,
            },
        };

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn temp_config_file() {
        let mut p = std::env::temp_dir();
        p.push("./config.toml");
        let mut file = match std::fs::File::create(p) {
            Ok(file) => file,
            Err(_) => {
                panic!("Failed to create temp config file");
            }
        };
        if let Err(e) = file.write_all(
            b"
[network]
port = 37771
max_connections = 10
[nostr]
port = 4444
max_ws_connections = 1000000
[rpc]
enable_rpc = true
[metrics]
enable_metrics = false
[log]
write_to_file = true
path = 'log.r7'
        ",
        ) {
            eprintln!("Failed to write to temp config file: {}", e);
        }
    }

    #[test]
    fn load_from_file_test() {
        temp_config_file();
        let loaded_from_file = Config::load_from_file("./config.toml").unwrap();
        let default = Config::default().unwrap();

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
            loaded_from_file.nostr.max_ws_connections,
            default.nostr.max_ws_connections
        );
        assert_eq!(loaded_from_file.nostr.port, default.nostr.port);

        assert_eq!(loaded_from_file.rpc.enable_grpc, default.rpc.enable_grpc);
        assert_eq!(loaded_from_file.rpc.grpc_port, default.rpc.grpc_port);
    }

    #[test]
    fn default_test() {
        let default = Config::default().unwrap();

        assert_eq!("log.r7", default.log.path);
        assert_eq!(true, default.log.write_to_file);

        assert_eq!(false, default.metrics.enable_metrics);

        assert_eq!(10, default.network.max_connections);
        assert_eq!(37771, default.network.port);

        assert_eq!(1000000, default.nostr.max_ws_connections);
        assert_eq!(4444, default.nostr.port);

        assert_eq!(true, default.rpc.enable_grpc);
        assert_eq!(9090, default.rpc.grpc_port);
    }
}
