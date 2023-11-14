use std::io;
// use tonic::{transport::Server, Request, Response, Status};

#[tokio::main]
pub async fn start(_port: u16) -> Result<(), io::Error> {
    // let addr = format!("[::1]:{}", port).parse().unwrap();
    // Server::builder().add_service().serve(addr).await;
    Ok(())
}
