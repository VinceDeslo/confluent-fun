use tonic::{Request, Response, Status };
use super::service::Service;
use crate::db::{ establish_connection, delete_user };
use crate::users::{
    DeleteUserRequest, 
    DeleteUserResponse,
};

pub fn delete_user_operation(
    service: &Service,
    request: Request<DeleteUserRequest>,
) -> Result<Response<DeleteUserResponse>, Status> {
    println!("start Delete User operation.");
    let conn = &mut establish_connection(&service.config);
    let req = request.get_ref();
    println!("Request payload: {:#?}", &req);

    let count = delete_user(conn, req.id);
    let reply = DeleteUserResponse {
        deleted: count as i32,
    };

    println!("end Delete User operation.");
    Ok(Response::new(reply))
}
