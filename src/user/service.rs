use crate::config::Config;
use tonic::{Request, Response, Status };
use crate::users::{
    users_service_server::UsersService,
    CreateUserRequest, 
    CreateUserResponse,
    ReadUserRequest, 
    ReadUserResponse,
    ReadUsersRequest, 
    ReadUsersResponse,
    UpdateUserRequest, 
    UpdateUserResponse, 
    DeleteUserRequest, 
    DeleteUserResponse
};

use super::{
    create::create_user_operation,
    read::{read_user_operation, read_users_operation}, 
    update::update_user_operation,
    delete::delete_user_operation,
};

#[derive(Debug)]
pub struct Service {
    pub config: Config,
}

#[tonic::async_trait]
impl UsersService for Service {
    async fn create_user(&self, request: Request<CreateUserRequest>)
    -> Result<Response<CreateUserResponse>, Status> {
        create_user_operation(self, request)
    }

    async fn read_user(&self, request: Request<ReadUserRequest>) 
    -> Result<Response<ReadUserResponse>, Status> {
        read_user_operation(self, request)
    }

    async fn read_users(&self, request: Request<ReadUsersRequest>) 
    -> Result<Response<ReadUsersResponse>, Status> {
        read_users_operation(self, request)
    }

    async fn update_user(&self, request: Request<UpdateUserRequest>)
    -> Result<Response<UpdateUserResponse>, Status> {
        update_user_operation(self, request)
    }

    async fn delete_user(&self, request: Request<DeleteUserRequest>)
    -> Result<Response<DeleteUserResponse>, Status> {
        delete_user_operation(self, request)
    }
}
