use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::config::Config;
use crate::models::{User, NewUser};
use crate::schema::users;

fn build_db_conn_str(conf: &Config) -> String {
    let mut conn_str = "".to_owned();
    conn_str.push_str(&conf.database_protocol);
    conn_str.push_str(&conf.database_user);
    conn_str.push_str(&conf.database_password);
    conn_str.push_str(&conf.database_url);
    conn_str.push_str(&conf.database_port);
    conn_str.push_str(&conf.database_name);
    return conn_str;
}

pub fn establish_connection(conf: &Config) -> PgConnection {
    let conn_str = build_db_conn_str(conf);
    println!("Establishing connection with: {}", &conn_str);
    PgConnection::establish(&conn_str[..])
        .unwrap_or_else(|_| panic!("Error connecting to {}", conf.database_url))
}

pub fn get_users(conn: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    users
        .filter(active.eq(true))
        .limit(10)
        .load::<User>(conn)
        .expect("Error loading users")
}

pub fn get_user(conn: &mut PgConnection, user_id: i32) -> User {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .first(conn)
        .expect("Error loading users")
}

pub fn create_user(
    conn: &mut PgConnection,
    name: &str,
    bio: &str,
) -> User {
    let new_user = NewUser { name, bio };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user.")
}

pub fn update_user(
    conn: &mut PgConnection, 
    id: i32,
    new_name: &str,
    new_bio: &str,
) -> User {
    use crate::schema::users::dsl::*;

    diesel::update(users.find(id))
        .set((name.eq(new_name), bio.eq(new_bio)))
        .get_result::<User>(conn)
        .expect("Error updating existing user.")
}

pub fn delete_user(conn: &mut PgConnection, user_id: i32) -> usize {
    use crate::schema::users::dsl::*;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .expect("Error deleting existing user.")
}

