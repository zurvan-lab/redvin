use std::io;
use tonic::transport::Server;

use crate::network;
use crate::network::network::network_server::NetworkServer;

use crate::nostr;
use crate::nostr::nostr::nostr_rpc_server::NostrRpcServer;

#[tokio::main]
pub async fn start(port: u16) -> Result<(), io::Error> {
    let addr = format!("[::1]:{}", port).parse().unwrap();

    let network_service = network::NetworkService::default();
    let nostr_rpc_service = nostr::NostrRpcService::default();

    Server::builder()
        .add_service(NetworkServer::new(network_service))
        .add_service(NostrRpcServer::new(nostr_rpc_service))
        .serve(addr)
        .await;

    Ok(())
}
