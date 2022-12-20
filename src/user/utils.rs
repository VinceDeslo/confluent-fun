use crate::users;
use crate::models;

pub fn convert_user_for_transport(user: &models::User) -> Option<users::User> {
    return Some(users::User::from(user.clone()))
}