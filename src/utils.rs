use crate::models::User;

pub fn list_users(users: Vec<User>) {
    for user in users {
        println!("{:#?}", user);
    }
}