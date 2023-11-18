use tonic::{Request, Response, Status};

use nostr::nostr_rpc_server::NostrRpc;
use nostr::{
    GetAllPublicKeysRequest, GetAllPublicKeysResponse, GetEventRequest, GetEventResponse,
    GetProfileRequest, GetProfileResponse,
};

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

    async fn get_profile(
        &self,
        request: Request<GetProfileRequest>,
    ) -> Result<Response<GetProfileResponse>, Status> {
        let _req = request.into_inner();

        let reply = GetProfileResponse {
            about: "".to_string(),
            avatar: "".to_string(),
            banner: "".to_string(),
            name: "".to_string(),
            nip5_address: vec!["".to_string()],
        };

        Ok(Response::new(reply))
    }

    async fn get_event(
        &self,
        request: Request<GetEventRequest>,
    ) -> Result<Response<GetEventResponse>, Status> {
        let _req = request.into_inner();

        let reply = GetEventResponse {
            author_npub: "".to_string(),
            content: "".to_string(),
            published_at: "".to_string(),
            kind: 0,
            sig: "".to_string(),
        };

        Ok(Response::new(reply))
    }
}
