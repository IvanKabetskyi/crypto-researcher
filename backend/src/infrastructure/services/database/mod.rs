use mongodb::{Client, Database};
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

pub struct MongoDatabase {
    pub db: Database,
}

static DB: OnceCell<Mutex<MongoDatabase>> = OnceCell::new();

pub async fn get_db() -> &'static Mutex<MongoDatabase> {
    if let Some(db) = DB.get() {
        return db;
    }

    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| String::from("mongodb://localhost:27017"));
    let db_name = std::env::var("MONGODB_DATABASE")
        .unwrap_or_else(|_| String::from("crypto_researcher"));

    tracing::info!("Connecting to MongoDB...");

    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to connect to MongoDB");

    tracing::info!("Connected to MongoDB successfully");

    let mongo_db = MongoDatabase {
        db: client.database(&db_name),
    };

    let _ = DB.set(Mutex::new(mongo_db));
    DB.get().expect("Failed to initialize database connection")
}
