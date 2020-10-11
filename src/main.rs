extern crate diesel;
extern crate post_server;

mod post_service;
use post_service::MyPostService;

use dotenv::dotenv;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr}
};

use tonic::transport::Server;

use blog_grpc::blog::post_service_server::PostServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let port = env::var("PORT").unwrap().parse()?;
    println!("listening on port {}", port);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), port);

    let post_service = MyPostService::default();
    Server::builder()
        .add_service(PostServiceServer::new(post_service))
        .serve(addr)
        .await?;

    Ok(())
}

