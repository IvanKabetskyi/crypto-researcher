use crate::domain::market::entities::MarketSnapshot;
use crate::domain::prediction::services::AnalysisService;
use crate::infrastructure::repositories::prediction::PredictionRepository;
use crate::infrastructure::services::bybit::BybitService;
use crate::infrastructure::services::news::CryptoRssService;
use crate::infrastructure::services::openrouter::AIService;

pub async fn run_analysis_use_case() {
    let pairs = std::env::var("WATCH_PAIRS").unwrap_or_else(|_| "BTCUSDT,ETHUSDT".into());
    let symbols: Vec<String> = pairs.split(',').map(|s| s.trim().to_string()).collect();

    tracing::info!("Starting analysis for pairs: {:?}", symbols);

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
            return;
        }
    };

    if tickers.is_empty() {
        tracing::error!("No tickers returned from Bybit - check pair names");
        return;
    }

    // Fetch klines
    let mut klines = std::collections::HashMap::new();
    for symbol in &symbols {
        match bybit_service.fetch_klines(symbol, "60", 3).await {
            Ok(symbol_klines) => {
                tracing::info!("Fetched {} klines for {}", symbol_klines.len(), symbol);
                klines.insert(symbol.clone(), symbol_klines);
            }
            Err(e) => {
                tracing::warn!("Failed to fetch klines for {}: {}", symbol, e);
            }
        }
    }

    // Fetch news (optional - continue without it)
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
    let raw_predictions = match ollama_service.analyze(&snapshot).await {
        Ok(preds) => {
            tracing::info!("Ollama returned {} predictions", preds.len());
            preds
        }
        Err(e) => {
            tracing::error!("Ollama analysis failed: {}", e);
            return;
        }
    };

    if raw_predictions.is_empty() {
        tracing::warn!("AI returned 0 predictions");
        return;
    }

    // Filter by confidence - use wider range to actually get results
    let filtered = AnalysisService::filter_by_confidence(&raw_predictions, 70.0, 100.0);
    tracing::info!(
        "After confidence filter: {} of {} predictions kept",
        filtered.len(),
        raw_predictions.len()
    );

    // Save to MongoDB
    let prediction_repository = PredictionRepository::new().await;

    for prediction in filtered {
        match prediction_repository.save_prediction(prediction).await {
            Ok(_) => {
                tracing::info!(
                    "Saved prediction: {} {} {:.1}% confidence",
                    prediction.get_symbol(),
                    prediction.get_direction(),
                    prediction.get_confidence()
                );
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
}

