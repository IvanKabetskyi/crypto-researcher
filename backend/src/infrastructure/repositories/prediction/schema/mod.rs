use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::domain::prediction::entities::Prediction;

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
    // Pipeline fields
    pub market_bias: Option<String>,
    pub setup_type: Option<String>,
    pub risk_decision: Option<String>,
    pub risk_reward_ratio: Option<f64>,
    pub execution_action: Option<String>,
    pub secondary_target: Option<f64>,
    pub invalidation: Option<f64>,
    pub position_size_pct: Option<f64>,
    pub review_agreed: Option<bool>,
    pub review_confidence: Option<f64>,
}

impl PredictionSchema {
    pub fn from_prediction(p: &Prediction) -> Self {
        Self {
            id: p.get_id(),
            symbol: p.get_symbol().to_string(),
            direction: p.get_direction().to_string(),
            confidence: p.get_confidence(),
            reasoning: p.get_reasoning().to_string(),
            entry_price: p.get_entry_price(),
            target_price: p.get_target_price(),
            stop_loss: p.get_stop_loss(),
            created_at: p.get_created_at(),
            outcome: p.get_outcome().map(String::from),
            actual_price_after: p.get_actual_price_after(),
            timeframe: p.get_timeframe().map(String::from),
            market_bias: p.get_market_bias().map(String::from),
            setup_type: p.get_setup_type().map(String::from),
            risk_decision: p.get_risk_decision().map(String::from),
            risk_reward_ratio: p.get_risk_reward_ratio(),
            execution_action: p.get_execution_action().map(String::from),
            secondary_target: p.get_secondary_target(),
            invalidation: p.get_invalidation(),
            position_size_pct: p.get_position_size_pct(),
            review_agreed: p.get_review_agreed(),
            review_confidence: p.get_review_confidence(),
        }
    }

    pub fn to_prediction(self) -> Prediction {
        Prediction::new(
            &self.symbol,
            &self.direction,
            self.confidence,
            &self.reasoning,
            self.entry_price,
            self.target_price,
            self.stop_loss,
            Some(self.id),
            Some(self.created_at),
            self.outcome,
            self.actual_price_after,
            self.timeframe,
        )
        .with_pipeline(
            self.market_bias,
            self.setup_type,
            self.risk_decision,
            self.risk_reward_ratio,
            self.execution_action,
            self.secondary_target,
            self.invalidation,
            self.position_size_pct,
            self.review_agreed,
            self.review_confidence,
        )
    }
}
