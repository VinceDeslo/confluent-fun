use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::config::Config;
use crate::models::User;

pub fn establish_connection(conf: &Config) -> PgConnection {
    PgConnection::establish(&conf.database_url[..])
        .unwrap_or_else(|_| panic!("Error connecting to {}", conf.database_url))
}

pub fn get_users(connection: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    users
        .filter(active.eq(true))
        .limit(10)
        .load::<User>(connection)
        .expect("Error loading users")
}

