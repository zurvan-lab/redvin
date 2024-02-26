use config::NetworkConfig;

use libp2p::{identity, PeerId};
use std::time::Duration;

struct Network {
    keys: identity::Keypair,
    peer_id: PeerId,
}

impl Network {
    #[tokio::main]
    pub async fn start(config: NetworkConfig) {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let net = Network {
            keys: local_key,
            peer_id: local_peer_id,
        };

        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(net.keys)
            .with_async_std()
            .with_tcp(
                libp2p::tcp::Config::default(),
                libp2p::tls::Config::new,
                libp2p::yamux::Config::default,
            )
            .unwrap()
            .with_behaviour(|_| libp2p::ping::Behaviour::default())
            .unwrap()
            .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(30)))
            .build();

        for b in config.bootstraps.iter() {
            swarm.listen_on(b.parse().unwrap()).unwrap();
        }
    }
}
