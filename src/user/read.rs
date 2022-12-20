use tonic::{Request, Response, Status };
use super::service::Service;
use crate::db::{ establish_connection, get_users, get_user };
use crate::user::utils::convert_user_for_transport;
use crate::users::{
    User,
    ReadUserRequest, 
    ReadUserResponse,
    ReadUsersRequest,
    ReadUsersResponse
};

pub fn read_user_operation(
    service: &Service,
    request: Request<ReadUserRequest>,
) -> Result<Response<ReadUserResponse>, Status> {
    println!("start Read User operation.");
    let conn = &mut establish_connection(&service.config);
    let req = request.get_ref();
    println!("Request payload: {:#?}", &req);

    let user = get_user(conn, req.id);
    let reply = ReadUserResponse {
        user: convert_user_for_transport(&user)
    };
    
    println!("end Read User operation.");
    Ok(Response::new(reply))
}

pub fn read_users_operation(
    service: &Service, 
    request: Request<ReadUsersRequest>
) -> Result<Response<ReadUsersResponse>, Status> {
    println!("start Read Users operation.");
    let conn = &mut establish_connection(&service.config);
    let req = request.get_ref();
    println!("Request payload: {:#?}", &req);

    let users = get_users(conn);
    let converted_users = users
        .iter()
        .map(|u| User::from(u.clone()))
        .collect();

    let reply = ReadUsersResponse {
        users: converted_users
    };
    
    println!("end Update User operation.");
    Ok(Response::new(reply))
}