use std::time::Duration;

use crate::application::dto::config_dto::ConfigDto;
use crate::application::request_dto::analyze_params_dto::AnalyzeParams;
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

            let default_pairs = ConfigDto::default_config().pairs.join(",");
            let pairs = std::env::var("WATCH_PAIRS").unwrap_or(default_pairs);
            let symbols: Vec<String> = pairs.split(',').map(|s| s.trim().to_string()).collect();
            let params = AnalyzeParams {
                pairs: symbols,
                timeframe: "1h".to_string(),
                min_confidence: 30.0,
            };
            if let Err(e) = run_analysis_use_case(params).await {
                tracing::error!("Scheduled analysis failed: {}", e);
            }

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

    let predictions = match prediction_repository.get_predictions(filter).await {
        Ok(preds) => preds,
        Err(_) => {
            tracing::error!("Failed to fetch past predictions for outcome check");
            return;
        }
    };

    let bybit_service = BybitService::new();

    let pending_predictions: Vec<_> = predictions
        .iter()
        .filter(|p| {
            let outcome = p.get_outcome();
            outcome.is_none() || outcome == Some("pending")
        })
        .collect();

    for prediction in pending_predictions {
        let symbol = prediction.get_symbol().to_string();
        let symbols = vec![symbol];
        let tickers_result = bybit_service.fetch_tickers(&symbols).await;

        if let Ok(tickers) = tickers_result {
            if let Some(ticker) = tickers.first() {
                let current_price = ticker.get_last_price();
                let outcome = AnalysisService::determine_outcome(prediction, current_price);

                if outcome != "pending" {
                    let update_result = prediction_repository
                        .update_outcome(prediction.get_id(), &outcome, current_price)
                        .await;

                    if let Err(_) = update_result {
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
