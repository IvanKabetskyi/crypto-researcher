use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct SymbolAccuracy {
    pub total: i64,
    pub correct: i64,
    pub incorrect: i64,
    pub pending: i64,
    pub accuracy_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct AccuracyDto {
    pub total_predictions: i64,
    pub correct: i64,
    pub incorrect: i64,
    pub pending: i64,
    pub accuracy_percentage: f64,
    pub by_symbol: HashMap<String, SymbolAccuracy>,
}

impl AccuracyDto {
    pub fn empty() -> Self {
        Self {
            total_predictions: 0,
            correct: 0,
            incorrect: 0,
            pending: 0,
            accuracy_percentage: 0.0,
            by_symbol: HashMap::new(),
        }
    }
}
