use serde::Deserialize;
use std::error::Error;
use toml::from_str;

#[derive(Debug, Deserialize)]
struct Config {
    log: LogConfig,
    rpc: RpcConfig,
    metrics: MetricsConfig,
    network: NetworkConfig,
    nostr: NostrConfig,
}

#[derive(Debug, Deserialize)]
struct LogConfig {
    write_to_file: bool,
    path: String,
}

#[derive(Debug, Deserialize)]
struct RpcConfig {
    enable_rpc: bool,
}

#[derive(Debug, Deserialize)]
struct MetricsConfig {
    enable_metrics: bool,
}

#[derive(Debug, Deserialize)]
struct NetworkConfig {
    port: u16,
    max_connections: u16,
}

#[derive(Debug, Deserialize)]
struct NostrConfig {
    port: u16,
    max_ws_connections: u32,
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
            rpc: RpcConfig { enable_rpc: true },
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
    use super::*;

    #[test]
    fn config() {
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

        assert_eq!(loaded_from_file.rpc.enable_rpc, default.rpc.enable_rpc);
    }
}
