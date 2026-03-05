use crate::application::dto::prediction_dto::PredictionDto;
use crate::application::request_dto::analyze_params_dto::AnalyzeParams;
use crate::domain::market::entities::MarketSnapshot;
use crate::infrastructure::repositories::prediction::PredictionRepository;
use crate::infrastructure::services::bybit::BybitService;
use crate::infrastructure::services::news::CryptoRssService;
use crate::infrastructure::services::openrouter::AIService;

fn map_timeframe_to_interval(timeframe: &str) -> &str {
    match timeframe {
        "5min" => "5",
        "30min" => "30",
        "1h" => "60",
        "6h" => "360",
        "12h" => "720",
        "24h" => "D",
        _ => "60",
    }
}

pub async fn run_analysis_use_case(params: AnalyzeParams) -> Vec<PredictionDto> {
    let symbols = params.pairs;
    let timeframe = &params.timeframe;
    let min_confidence = params.min_confidence;
    let kline_interval = map_timeframe_to_interval(timeframe);

    tracing::info!("Starting analysis for pairs: {:?}, timeframe: {}", symbols, timeframe);

    let bybit_service = BybitService::new();
    let news_service = CryptoRssService::new();
    let ollama_service = AIService::new();

    // Fetch tickers
    let tickers = match bybit_service.fetch_tickers(&symbols).await {
        Ok(t) => {
            tracing::info!("Fetched {} tickers from Bybit", t.len());
            t
        }
        Err(e) => {
            tracing::error!("Failed to fetch tickers from Bybit: {}", e);
            return vec![];
        }
    };

    if tickers.is_empty() {
        tracing::error!("No tickers returned from Bybit - check pair names");
        return vec![];
    }

    // Fetch klines
    let mut klines = std::collections::HashMap::new();
    for symbol in &symbols {
        match bybit_service.fetch_klines(symbol, kline_interval, 3).await {
            Ok(symbol_klines) => {
                tracing::info!("Fetched {} klines for {}", symbol_klines.len(), symbol);
                klines.insert(symbol.clone(), symbol_klines);
            }
            Err(e) => {
                tracing::warn!("Failed to fetch klines for {}: {}", symbol, e);
            }
        }
    }

    // Fetch news
    let currencies: Vec<String> = symbols.iter().map(|s| s.replace("USDT", "")).collect();
    let news = match news_service.fetch_news(&currencies).await {
        Ok(articles) => {
            tracing::info!("Fetched {} news articles", articles.len());
            articles
        }
        Err(e) => {
            tracing::warn!("Failed to fetch news (continuing without): {}", e);
            Vec::new()
        }
    };

    let snapshot = MarketSnapshot::new(tickers, klines, news);

    // Call AI for analysis
    let raw_predictions = match ollama_service.analyze(&snapshot, timeframe).await {
        Ok(preds) => {
            tracing::info!("AI returned {} predictions", preds.len());
            preds
        }
        Err(e) => {
            tracing::error!("AI analysis failed: {}", e);
            return vec![];
        }
    };

    if raw_predictions.is_empty() {
        tracing::warn!("AI returned 0 predictions");
        return vec![];
    }

    // Filter by confidence
    let filtered: Vec<_> = raw_predictions
        .iter()
        .filter(|p| p.get_confidence() >= min_confidence && p.get_confidence() <= 100.0)
        .collect();

    tracing::info!(
        "After confidence filter (>= {}): {} of {} predictions kept",
        min_confidence,
        filtered.len(),
        raw_predictions.len()
    );

    // Save to MongoDB and collect DTOs
    let prediction_repository = PredictionRepository::new().await;
    let mut saved_dtos: Vec<PredictionDto> = Vec::new();

    for prediction in filtered {
        match prediction_repository.save_prediction(prediction).await {
            Ok(saved) => {
                tracing::info!(
                    "Saved prediction: {} {} {:.1}% confidence",
                    prediction.get_symbol(),
                    prediction.get_direction(),
                    prediction.get_confidence()
                );
                saved_dtos.push(PredictionDto::transform_entity(saved));
            }
            Err(e) => {
                tracing::error!(
                    "Failed to save prediction for {}: {}",
                    prediction.get_symbol(),
                    e.message
                );
            }
        }
    }

    saved_dtos
}
