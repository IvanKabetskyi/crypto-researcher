use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PredictionSchema {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub symbol: String,
    pub direction: String,
    pub confidence: f64,
    pub reasoning: String,
    pub entry_price: f64,
    pub target_price: f64,
    pub stop_loss: f64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    pub outcome: Option<String>,
    pub actual_price_after: Option<f64>,
    pub timeframe: Option<String>,
}
