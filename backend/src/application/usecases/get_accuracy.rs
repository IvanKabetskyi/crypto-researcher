use std::collections::HashMap;

use crate::infrastructure::repositories::prediction::PredictionRepository;

use crate::application::dto::accuracy_dto::{AccuracyDto, SymbolAccuracy};
use crate::application::error::DataError;

pub async fn get_accuracy_use_case() -> Result<AccuracyDto, DataError> {
    let prediction_repository = PredictionRepository::new().await;
    let stats_response = prediction_repository.get_accuracy_stats().await;

    if stats_response.is_err() {
        return Err(stats_response.err().unwrap());
    }

    let stats = stats_response.unwrap();

    let accuracy_percentage = if stats.total > 0 && (stats.correct + stats.incorrect) > 0 {
        (stats.correct as f64 / (stats.correct + stats.incorrect) as f64) * 100.0
    } else {
        0.0
    };

    let mut by_symbol: HashMap<String, SymbolAccuracy> = HashMap::new();

    for (symbol, symbol_stats) in stats.by_symbol.iter() {
        let symbol_accuracy = if (symbol_stats.correct + symbol_stats.incorrect) > 0 {
            (symbol_stats.correct as f64 / (symbol_stats.correct + symbol_stats.incorrect) as f64)
                * 100.0
        } else {
            0.0
        };

        by_symbol.insert(
            symbol.clone(),
            SymbolAccuracy {
                total: symbol_stats.total,
                correct: symbol_stats.correct,
                incorrect: symbol_stats.incorrect,
                pending: symbol_stats.pending,
                accuracy_percentage: symbol_accuracy,
            },
        );
    }

    Ok(AccuracyDto {
        total_predictions: stats.total,
        correct: stats.correct,
        incorrect: stats.incorrect,
        pending: stats.pending,
        accuracy_percentage,
        by_symbol,
    })
}
