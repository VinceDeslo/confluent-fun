mod config;
mod db;
pub mod models;
pub mod schema;
pub mod user;
pub mod users {
    tonic::include_proto!("users");
}

use tonic::transport::Server;
use crate::config::load_config;
use crate::user::service::Service;
use users::users_service_server::UsersServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let config = load_config();
    let service = Service { config };

    println!("Users Service started with config: {:#?}", &service.config);

    Server::builder()
        .add_service(UsersServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}