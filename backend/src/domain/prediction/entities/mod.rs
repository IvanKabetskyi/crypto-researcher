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
    // Pipeline fields
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
    // Market context fields
    trend_strength: Option<String>,
    momentum: Option<String>,
    volume_profile: Option<String>,
    derivatives_sentiment: Option<String>,
    prediction_status: Option<String>,
}

impl Prediction {
    #[allow(clippy::too_many_arguments)]
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
            market_bias: None,
            setup_type: None,
            risk_decision: None,
            risk_reward_ratio: None,
            execution_action: None,
            secondary_target: None,
            invalidation: None,
            position_size_pct: None,
            review_agreed: None,
            review_confidence: None,
            review_verdict: None,
            review_decision: None,
            review_issues: None,
            review_notes: None,
            trend_strength: None,
            momentum: None,
            volume_profile: None,
            derivatives_sentiment: None,
            prediction_status: None,
        }
    }

    pub fn with_pipeline(
        mut self,
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
        trend_strength: Option<String>,
        momentum: Option<String>,
        volume_profile: Option<String>,
        derivatives_sentiment: Option<String>,
        prediction_status: Option<String>,
    ) -> Self {
        self.market_bias = market_bias;
        self.setup_type = setup_type;
        self.risk_decision = risk_decision;
        self.risk_reward_ratio = risk_reward_ratio;
        self.execution_action = execution_action;
        self.secondary_target = secondary_target;
        self.invalidation = invalidation;
        self.position_size_pct = position_size_pct;
        self.review_agreed = review_agreed;
        self.review_confidence = review_confidence;
        self.review_verdict = review_verdict;
        self.review_decision = review_decision;
        self.review_issues = review_issues;
        self.review_notes = review_notes;
        self.trend_strength = trend_strength;
        self.momentum = momentum;
        self.volume_profile = volume_profile;
        self.derivatives_sentiment = derivatives_sentiment;
        self.prediction_status = prediction_status;
        self
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

    pub fn get_market_bias(&self) -> Option<&str> {
        self.market_bias.as_deref()
    }

    pub fn get_setup_type(&self) -> Option<&str> {
        self.setup_type.as_deref()
    }

    pub fn get_risk_decision(&self) -> Option<&str> {
        self.risk_decision.as_deref()
    }

    pub fn get_risk_reward_ratio(&self) -> Option<f64> {
        self.risk_reward_ratio
    }

    pub fn get_execution_action(&self) -> Option<&str> {
        self.execution_action.as_deref()
    }

    pub fn get_secondary_target(&self) -> Option<f64> {
        self.secondary_target
    }

    pub fn get_invalidation(&self) -> Option<f64> {
        self.invalidation
    }

    pub fn get_position_size_pct(&self) -> Option<f64> {
        self.position_size_pct
    }

    pub fn get_review_agreed(&self) -> Option<bool> {
        self.review_agreed
    }

    pub fn get_review_confidence(&self) -> Option<f64> {
        self.review_confidence
    }

    pub fn get_review_verdict(&self) -> Option<&str> {
        self.review_verdict.as_deref()
    }

    pub fn get_review_decision(&self) -> Option<&str> {
        self.review_decision.as_deref()
    }

    pub fn get_review_issues(&self) -> Option<&[String]> {
        self.review_issues.as_deref()
    }

    pub fn get_review_notes(&self) -> Option<&[String]> {
        self.review_notes.as_deref()
    }

    pub fn get_trend_strength(&self) -> Option<&str> {
        self.trend_strength.as_deref()
    }

    pub fn get_momentum(&self) -> Option<&str> {
        self.momentum.as_deref()
    }

    pub fn get_volume_profile(&self) -> Option<&str> {
        self.volume_profile.as_deref()
    }

    pub fn get_derivatives_sentiment(&self) -> Option<&str> {
        self.derivatives_sentiment.as_deref()
    }

    pub fn get_prediction_status(&self) -> Option<&str> {
        self.prediction_status.as_deref()
    }
}
