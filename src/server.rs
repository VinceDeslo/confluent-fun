mod config;
mod db;
pub mod models;
pub mod schema;
pub mod users {
    tonic::include_proto!("users");
}

use tonic::{transport::Server, Request, Response, Status };

use users::users_service_server::{UsersService, UsersServiceServer};
use users::{
    CreateUserRequest, CreateUserResponse,
    // ReadUserRequest, ReadUserResponse,
    // UpdateUserRequest, UpdateUserResponse, 
    // DeleteUserRequest, DeleteUserResponse,
};

use crate::config::{Config, load_config};
use crate::db::{
    establish_connection,
    create_user,
};

#[derive(Debug)]
pub struct Service {
    config: Config,
}

#[tonic::async_trait]
impl UsersService for Service {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        println!("start Create User operation.");
        let conn = &mut establish_connection(&self.config);
        let req = request.get_ref();
        println!("Request payload: {:#?}", &req);

        let user = create_user(conn, &req.name, &req.bio);
        let reply = users::CreateUserResponse {
            id: user.id,
        };

        println!("end Create User operation.");
        Ok(Response::new(reply))
    }
}

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