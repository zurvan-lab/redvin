use tonic::{Request, Response, Status};

use network::network_server::Network;
use network::{
    GetNetworkInfoRequest, GetNetworkInfoResponse, GetNodeInfoRequest, GetNodeInfoResponse,
    NetworkHealthRequest, NetworkHealthResponse, PeerInfo,
};

pub mod network {
    tonic::include_proto!("network");
}

#[derive(Debug, Default)]
pub struct NetworkService {}

#[tonic::async_trait]
impl Network for NetworkService {
    async fn get_network_info(
        &self,
        request: Request<GetNetworkInfoRequest>,
    ) -> Result<Response<GetNetworkInfoResponse>, Status> {
        let _req = request.into_inner();

        let reply = GetNetworkInfoResponse {
            total_sent_bytes: 0,
            total_received_bytes: 0,
            started_at: 0,
            peers: vec![PeerInfo {
                address: String::from(""),
                agent: String::from(""),
                moniker: String::from(""),
                last_received: 0,
                last_sent: 0,
                status: 0,
                peer_id: Vec::new(),
            }],
        };

        Ok(Response::new(reply))
    }

    async fn get_node_info(
        &self,
        request: Request<GetNodeInfoRequest>,
    ) -> Result<Response<GetNodeInfoResponse>, Status> {
        let _req = request.into_inner();

        let reply = GetNodeInfoResponse {
            moniker: String::from(""),
            agent: String::from(""),
            peer_id: Vec::new(),
        };

        Ok(Response::new(reply))
    }

    async fn network_health(
        &self,
        request: Request<NetworkHealthRequest>,
    ) -> Result<Response<NetworkHealthResponse>, Status> {
        let _req = request.into_inner();

        let reply = NetworkHealthResponse { is_healthy: false };

        Ok(Response::new(reply))
    }
}
