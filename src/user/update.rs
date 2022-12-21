use tonic::{Request, Response, Status };
use super::service::Service;
use crate::db::{ establish_connection, update_user };
use crate::user::utils::convert_user_for_transport;
use crate::users::{
    UpdateUserResponse, 
    UpdateUserRequest
};
use crate::events::update_user::update_user_event;

pub fn update_user_operation(
    service: &Service,
    request: Request<UpdateUserRequest>,
) -> Result<Response<UpdateUserResponse>, Status> {
    println!("start Update User operation.");
    let conn = &mut establish_connection(&service.config);
    let req = request.get_ref();
    println!("Request payload: {:#?}", &req);

    let user = update_user(conn, req.id, &req.name, &req.bio);
    update_user_event(service, &user);

    let reply = UpdateUserResponse {
        user: convert_user_for_transport(&user),
    };

    println!("end Update User operation.");
    Ok(Response::new(reply))
}