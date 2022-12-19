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
    ReadUserRequest, ReadUserResponse,
    ReadUsersRequest, ReadUsersResponse,
    UpdateUserRequest, UpdateUserResponse, 
    DeleteUserRequest, DeleteUserResponse
};

use crate::config::{Config, load_config};
use crate::db::{
    establish_connection,
    create_user,
    get_users,
    get_user,
    update_user,
    delete_user
};

#[derive(Debug)]
pub struct Service {
    config: Config,
}

impl From<models::User> for users::User {
    fn from(p: models::User) -> Self {
        Self {
          id: p.id,
          name: p.name,
          bio: p.bio,
          active: p.active
        }
      }
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

    async fn read_user(
        &self,
        request: Request<ReadUserRequest>,
    ) -> Result<Response<ReadUserResponse>, Status> {
        println!("start Read User operation.");
        let conn = &mut establish_connection(&self.config);
        let req = request.get_ref();
        println!("Request payload: {:#?}", &req);

        let user = get_user(conn, req.id);
        let reply = users::ReadUserResponse {
            id: user.id,
            name: user.name,
            bio: user.bio,
            active: user.active,
        };

        println!("end Read User operation.");
        Ok(Response::new(reply))
    }

    async fn read_users(&self, request: Request<ReadUsersRequest>) -> Result<Response<ReadUsersResponse>, Status> {
        println!("start Read Users operation.");
        let conn = &mut establish_connection(&self.config);
        let req = request.get_ref();
        println!("Request payload: {:#?}", &req);

        let users = get_users(conn);
        let converted_users: Vec<users::User> = users
            .iter()
            .map(|u| users::User::from(u.clone()))
            .collect();
        let reply = users::ReadUsersResponse {
            users: converted_users
        };

        println!("end Update User operation.");
        Ok(Response::new(reply))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserResponse>, Status> {
        println!("start Update User operation.");
        let conn = &mut establish_connection(&self.config);
        let req = request.get_ref();
        println!("Request payload: {:#?}", &req);

        let user = update_user(conn, req.id, &req.name, &req.bio);
        let reply = users::UpdateUserResponse {
            id: user.id,
            name: user.name,
            bio: user.bio,
            active: user.active,
        };

        println!("end Update User operation.");
        Ok(Response::new(reply))
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        println!("start Delete User operation.");
        let conn = &mut establish_connection(&self.config);
        let req = request.get_ref();
        println!("Request payload: {:#?}", &req);

        let count = delete_user(conn, req.id);
        let reply = users::DeleteUserResponse {
            deleted: count as i32,
        };

        println!("end Delete User operation.");
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