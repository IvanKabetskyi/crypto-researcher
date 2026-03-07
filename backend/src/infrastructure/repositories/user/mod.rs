mod schema;

use mongodb::bson::doc;
use mongodb::Collection;

use crate::application::error::DataError;
use crate::domain::user::entities::User;
use crate::infrastructure::services::database::get_db;

pub use schema::UserSchema;

pub struct UserRepository {
    collection: Collection<UserSchema>,
}

impl UserRepository {
    pub async fn new() -> Self {
        let database = get_db().await.lock().await;
        Self {
            collection: database.db.collection::<UserSchema>("users"),
        }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, DataError> {
        let pattern = format!(
            "^{}$",
            email.replace('.', "\\.").replace('+', "\\+")
        );
        let result = self
            .collection
            .find_one(
                Some(doc! { "email": { "$regex": &pattern, "$options": "i" } }),
                None,
            )
            .await
            .map_err(|_| DataError::new("failed to find user"))?;

        Ok(result.map(|s| s.to_user()))
    }

    pub async fn create_user(&self, user: &User) -> Result<(), DataError> {
        let document = UserSchema::from_user(user);
        self.collection
            .insert_one(document, None)
            .await
            .map_err(|_| DataError::new("failed to create user"))?;
        Ok(())
    }
}
