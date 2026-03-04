use async_std::sync::Mutex;
use async_std::task;
use mongodb::{Client, Database};
use once_cell::sync::Lazy;

pub struct MongoDatabase {
    pub db: Database,
}

impl MongoDatabase {
    pub fn init() -> Self {
        let uri = std::env::var("MONGODB_URI")
            .unwrap_or_else(|_| String::from("mongodb://localhost:27017"));
        let db_name = std::env::var("MONGODB_DATABASE")
            .unwrap_or_else(|_| String::from("crypto_researcher"));

        let client = task::block_on(Client::with_uri_str(uri)).unwrap();

        Self {
            db: client.database(&db_name),
        }
    }
}

pub static DB: Lazy<Mutex<MongoDatabase>> = Lazy::new(|| Mutex::new(MongoDatabase::init()));
