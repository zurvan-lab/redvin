use tonic::{Request, Response, Status};

use network::network_server::Network;
use network::{
    GetNetworkInfoRequest, GetNetworkInfoResponse, GetNodeInfoRequest, GetNodeInfoResponse,
    PeerInfo,
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
        let req = request.into_inner();

        let reply = GetNetworkInfoResponse {
            total_sent_bytes: 0,
            total_received_bytes: 0,
            started_at: 0,
            peers: vec![PeerInfo {
                address: "".to_string(),
                agent: "".to_string(),
                moniker: "".to_string(),
                last_received: 0,
                last_sent: 0,
                status: 0,
                peer_id: vec![0],
            }],
        };

        Ok(Response::new(reply))
    }

    async fn get_node_info(
        &self,
        request: Request<GetNodeInfoRequest>,
    ) -> Result<Response<GetNodeInfoResponse>, Status> {
        let req = request.into_inner();

        let reply = GetNodeInfoResponse {
            moniker: "".to_string(),
            agent: "".to_string(),
            peer_id: vec![0],
        };

        Ok(Response::new(reply))
    }
}
