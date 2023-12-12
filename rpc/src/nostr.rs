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
            public_keys: Vec::new(),
        };

        Ok(Response::new(reply))
    }

    async fn get_profile(
        &self,
        request: Request<GetProfileRequest>,
    ) -> Result<Response<GetProfileResponse>, Status> {
        let _req = request.into_inner();

        let reply = GetProfileResponse {
            about: String::from(""),
            avatar: String::from(""),
            banner: String::from(""),
            name: String::from(""),
            nip5_address: String::from(""),
            ln_address: String::from(""),
        };

        Ok(Response::new(reply))
    }

    async fn get_event(
        &self,
        request: Request<GetEventRequest>,
    ) -> Result<Response<GetEventResponse>, Status> {
        let _req = request.into_inner();

        let reply = GetEventResponse {
            author_npub: String::from(""),
            content: String::from(""),
            published_at: String::from(""),
            kind: 0,
            sig: String::from(""),
        };

        Ok(Response::new(reply))
    }
}
