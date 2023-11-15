use tonic::{Request, Response, Status};

use nostr::nostr_rpc_server::NostrRpc;
use nostr::{GetAllPublicKeysRequest, GetAllPublicKeysResponse};

pub mod nostr {
    tonic::include_proto!("nostr");
}

#[derive(Debug, Default)]
pub struct NostrRpcService {}

#[tonic::async_trait]
impl NostrRpc for NostrRpcService {
    async fn get_all_public_keys(
        &self,
        request: Request<GetAllPublicKeysRequest>,
    ) -> Result<Response<GetAllPublicKeysResponse>, Status> {
        let _req = request.into_inner();

        let reply = GetAllPublicKeysResponse {
            public_keys: vec!["".to_string()],
        };

        Ok(Response::new(reply))
    }
}
