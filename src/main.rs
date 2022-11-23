#![allow(dead_code)]
#![allow(unused_variables)]

use log::{info};

mod config;
mod db;
mod utils;
pub mod models;
pub mod schema;

use crate::config::load_config;
use crate::db::establish_connection;
use crate::db::{create_user, delete_user, get_users, update_user};
use crate::utils::list_users;

fn main() {
    env_logger::init();
    let conf = load_config();
    info!("Program running with: {:#?}", conf);
   
    // Establish RDS Postgres connection
    let conn = &mut establish_connection(&conf);

    // Create users
    let user = create_user(conn, "Vince", "Software Engineer");
    info!("Created user: {}", user.name);

    // Read user
    let users = get_users(conn);
    info!("Retrieved users: {} ", users.len());
    list_users(users);

    // Update user bio
    let user = update_user(
        conn, 
        user.id, 
        &user.name[..], 
        "Senior Software Engineer"
    );

    // Read user
    let users = get_users(conn);
    info!("Retrieved users: {} ", users.len());
    list_users(users);

    // Delete user
    let deleted = delete_user(conn, user.id);
    info!("Deleted users: {}", deleted);
}
