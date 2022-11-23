#![allow(dead_code)]
#![allow(unused_variables)]

use log::{info};

mod config;
mod db;
pub mod models;
pub mod schema;

use crate::config::load_config;
use crate::db::establish_connection;
use crate::db::get_users;

fn main() {
    env_logger::init();
    let conf = load_config();
    info!("Program running with: {:#?}", conf);
   
    let connection = &mut establish_connection(&conf);
    let users = get_users(connection);
    info!("Retrieved {} users", users.len());
}
