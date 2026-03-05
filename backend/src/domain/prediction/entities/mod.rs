use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;

pub struct Prediction {
    id: ObjectId,
    symbol: String,
    direction: String,
    confidence: f64,
    reasoning: String,
    entry_price: f64,
    target_price: f64,
    stop_loss: f64,
    created_at: DateTime<Utc>,
    outcome: Option<String>,
    actual_price_after: Option<f64>,
    timeframe: Option<String>,
}

impl Prediction {
    pub fn new(
        symbol: &str,
        direction: &str,
        confidence: f64,
        reasoning: &str,
        entry_price: f64,
        target_price: f64,
        stop_loss: f64,
        id: Option<ObjectId>,
        created_at: Option<DateTime<Utc>>,
        outcome: Option<String>,
        actual_price_after: Option<f64>,
        timeframe: Option<String>,
    ) -> Self {
        Self {
            id: id.unwrap_or(ObjectId::new()),
            symbol: symbol.into(),
            direction: direction.into(),
            confidence,
            reasoning: reasoning.into(),
            entry_price,
            target_price,
            stop_loss,
            created_at: created_at.unwrap_or(Utc::now()),
            outcome,
            actual_price_after,
            timeframe,
        }
    }

    pub fn get_id(&self) -> ObjectId {
        self.id
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_direction(&self) -> &str {
        &self.direction
    }

    pub fn get_confidence(&self) -> f64 {
        self.confidence
    }

    pub fn get_reasoning(&self) -> &str {
        &self.reasoning
    }

    pub fn get_entry_price(&self) -> f64 {
        self.entry_price
    }

    pub fn get_target_price(&self) -> f64 {
        self.target_price
    }

    pub fn get_stop_loss(&self) -> f64 {
        self.stop_loss
    }

    pub fn get_created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn get_outcome(&self) -> Option<&str> {
        self.outcome.as_deref()
    }

    pub fn get_actual_price_after(&self) -> Option<f64> {
        self.actual_price_after
    }

    pub fn get_timeframe(&self) -> Option<&str> {
        self.timeframe.as_deref()
    }
}
