use tonic::{transport::Server, Request, Response, Status };

pub mod users {
    tonic::include_proto!("users");
}

use users::users_service_server::{UsersService, UsersServiceServer};
use users::{
    CreateUserRequest, CreateUserResponse,
    // ReadUserRequest, ReadUserResponse,
    // UpdateUserRequest, UpdateUserResponse, 
    // DeleteUserRequest, DeleteUserResponse,
};

#[derive(Debug, Default)]
pub struct Service {}

#[tonic::async_trait]
impl UsersService for Service {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {

        let reply = users::CreateUserResponse {
            id: 1,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = Service::default();

    Server::builder()
        .add_service(UsersServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}