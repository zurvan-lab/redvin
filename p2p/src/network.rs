use config::NetworkConfig;
use libp2p::kad;
use libp2p::kad::store::MemoryStore;
use libp2p::mdns;

pub struct Network {
    pub config: NetworkConfig,
    pub kademlia: kad::Behaviour<MemoryStore>,
    pub mdns: mdns::async_io::Behaviour,
}

pub fn start(_config: NetworkConfig) {}
