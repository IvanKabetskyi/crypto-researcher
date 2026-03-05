use mongodb::bson::oid::ObjectId;

pub struct User {
    id: ObjectId,
    email: String,
    password_hash: String,
}

impl User {
    pub fn new(email: &str, password_hash: &str, id: Option<ObjectId>) -> Self {
        Self {
            id: id.unwrap_or(ObjectId::new()),
            email: email.into(),
            password_hash: password_hash.into(),
        }
    }

    pub fn get_id(&self) -> ObjectId {
        self.id
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_password_hash(&self) -> &str {
        &self.password_hash
    }
}
