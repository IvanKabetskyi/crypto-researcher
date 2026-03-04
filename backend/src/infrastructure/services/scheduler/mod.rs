use std::time::Duration;

use crate::application::usecases::run_analysis::run_analysis_use_case;
use crate::domain::prediction::services::AnalysisService;
use crate::infrastructure::repositories::prediction::PredictionRepository;
use crate::infrastructure::services::bybit::BybitService;

pub fn start_scheduler(interval_secs: u64) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

        loop {
            interval.tick().await;

            tracing::info!("Running scheduled analysis cycle");

            run_analysis_use_case().await;

            check_past_predictions().await;
        }
    });
}

async fn check_past_predictions() {
    let prediction_repository = PredictionRepository::new().await;

    let filter = crate::application::request_dto::filter_params_dto::FilterParams {
        symbol: None,
        min_confidence: None,
        direction: None,
        limit: Some(100),
    };

    let predictions_response = prediction_repository.get_predictions(filter).await;

    if predictions_response.is_err() {
        tracing::error!("Failed to fetch past predictions for outcome check");
        return;
    }

    let predictions = predictions_response.unwrap();

    let bybit_service = BybitService::new();

    let pending_predictions: Vec<&crate::domain::prediction::entities::Prediction> = predictions
        .iter()
        .filter(|p| {
            let outcome = p.get_outcome();
            outcome.is_none() || outcome.as_deref() == Some("pending")
        })
        .collect();

    for prediction in pending_predictions {
        let symbol = prediction.get_symbol();
        let symbols = vec![symbol.clone()];
        let tickers_result = bybit_service.fetch_tickers(&symbols).await;

        if let Ok(tickers) = tickers_result {
            if let Some(ticker) = tickers.first() {
                let current_price = ticker.get_last_price();
                let outcome = AnalysisService::determine_outcome(prediction, current_price);

                if outcome != "pending" {
                    let update_result = prediction_repository
                        .update_outcome(prediction.get_id(), outcome.clone(), current_price)
                        .await;

                    if update_result.is_err() {
                        tracing::error!(
                            "Failed to update outcome for prediction {}",
                            prediction.get_id()
                        );
                    } else {
                        tracing::info!(
                            "Updated prediction {} outcome: {}",
                            prediction.get_id(),
                            outcome
                        );
                    }
                }
            }
        }
    }
}
