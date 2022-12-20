mod config;
mod db;
pub mod models;
pub mod schema;
pub mod user;
pub mod events;
pub mod users {
    tonic::include_proto!("users");
}

use tonic::transport::Server;
use crate::config::load_config;
use crate::user::service::Service;
use crate::events::producer::EventProducer;
use users::users_service_server::UsersServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let config = load_config();
    let event_producer = EventProducer::new(&config);
    let service = Service { event_producer, config };

    println!("Users Service started with config: {:#?}", &service.config);

    Server::builder()
        .add_service(UsersServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}