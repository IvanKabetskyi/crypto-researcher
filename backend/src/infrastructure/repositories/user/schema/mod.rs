use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::domain::user::entities::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSchema {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub email: String,
    pub password_hash: String,
}

impl UserSchema {
    pub fn from_user(u: &User) -> Self {
        Self {
            id: u.get_id(),
            email: u.get_email().to_string(),
            password_hash: u.get_password_hash().to_string(),
        }
    }

    pub fn to_user(self) -> User {
        User::new(&self.email, &self.password_hash, Some(self.id))
    }
}
