use crate::models;
use crate::users;

impl From<models::User> for users::User {
    fn from(u: models::User) -> Self {
        Self {
          id: u.id,
          name: u.name,
          bio: u.bio,
          active: u.active
        }
      }
}