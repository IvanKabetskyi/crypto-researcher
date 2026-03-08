use serde::Serialize;

use crate::domain::prediction::entities::Prediction;

#[derive(Debug, Serialize)]
pub struct PredictionDto {
    id: String,
    symbol: String,
    direction: String,
    confidence: f64,
    reasoning: String,
    entry_price: f64,
    target_price: f64,
    stop_loss: f64,
    created_at: String,
    outcome: Option<String>,
    timeframe: Option<String>,
    market_bias: Option<String>,
    setup_type: Option<String>,
    risk_decision: Option<String>,
    risk_reward_ratio: Option<f64>,
    execution_action: Option<String>,
    secondary_target: Option<f64>,
    invalidation: Option<f64>,
    position_size_pct: Option<f64>,
    review_agreed: Option<bool>,
    review_confidence: Option<f64>,
    review_verdict: Option<String>,
    review_decision: Option<String>,
    review_issues: Option<Vec<String>>,
    review_notes: Option<Vec<String>>,
}

impl PredictionDto {
    pub fn transform_entity(prediction: Prediction) -> Self {
        Self {
            id: prediction.get_id().to_hex(),
            symbol: prediction.get_symbol().to_string(),
            direction: prediction.get_direction().to_string(),
            confidence: prediction.get_confidence(),
            reasoning: prediction.get_reasoning().to_string(),
            entry_price: prediction.get_entry_price(),
            target_price: prediction.get_target_price(),
            stop_loss: prediction.get_stop_loss(),
            created_at: prediction.get_created_at().to_rfc3339(),
            outcome: prediction.get_outcome().map(String::from),
            timeframe: prediction.get_timeframe().map(String::from),
            market_bias: prediction.get_market_bias().map(String::from),
            setup_type: prediction.get_setup_type().map(String::from),
            risk_decision: prediction.get_risk_decision().map(String::from),
            risk_reward_ratio: prediction.get_risk_reward_ratio(),
            execution_action: prediction.get_execution_action().map(String::from),
            secondary_target: prediction.get_secondary_target(),
            invalidation: prediction.get_invalidation(),
            position_size_pct: prediction.get_position_size_pct(),
            review_agreed: prediction.get_review_agreed(),
            review_confidence: prediction.get_review_confidence(),
            review_verdict: prediction.get_review_verdict().map(String::from),
            review_decision: prediction.get_review_decision().map(String::from),
            review_issues: prediction.get_review_issues().map(|v| v.to_vec()),
            review_notes: prediction.get_review_notes().map(|v| v.to_vec()),
        }
    }
}
