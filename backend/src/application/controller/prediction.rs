use actix_web::{get, post, web::Query, web::Json, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::application::dto::accuracy_dto::AccuracyDto;
use crate::application::dto::market_dto::MarketDto;
use crate::application::dto::prediction_dto::PredictionDto;
use crate::application::request_dto::analyze_params_dto::AnalyzeParams;
use crate::application::request_dto::filter_params_dto::FilterParams;

use crate::application::usecases::get_accuracy::get_accuracy_use_case;
use crate::application::usecases::get_predictions::get_predictions_use_case;
use crate::application::usecases::get_market::get_market_use_case;
use crate::application::usecases::run_analysis::run_analysis_use_case;

#[get("/api/predictions")]
pub async fn get_predictions(
    query: Query<FilterParams>,
) -> HttpResponse {
    let filter = query.into_inner();

    match get_predictions_use_case(filter).await {
        Ok(predictions) => HttpResponse::Ok().json(predictions),
        Err(e) => {
            tracing::warn!("Failed to get predictions (MongoDB may be down): {}", e.message);
            HttpResponse::Ok().json(Vec::<PredictionDto>::new())
        }
    }
}

#[get("/api/predictions/accuracy")]
pub async fn get_accuracy() -> HttpResponse {
    match get_accuracy_use_case().await {
        Ok(accuracy) => HttpResponse::Ok().json(accuracy),
        Err(e) => {
            tracing::warn!("Failed to get accuracy: {}", e.message);
            HttpResponse::Ok().json(AccuracyDto::empty())
        }
    }
}

#[get("/api/market")]
pub async fn get_market() -> HttpResponse {
    match get_market_use_case().await {
        Ok(market_data) => HttpResponse::Ok().json(market_data),
        Err(e) => {
            tracing::warn!("Failed to get market data: {}", e.message);
            HttpResponse::Ok().json(Vec::<MarketDto>::new())
        }
    }
}

#[post("/api/analyze")]
pub async fn trigger_analysis(body: Json<AnalyzeParams>) -> HttpResponse {
    let params = body.into_inner();
    tracing::info!("Manual analysis triggered from frontend: pairs={:?}, timeframe={}", params.pairs, params.timeframe);
    let predictions = run_analysis_use_case(params).await;
    HttpResponse::Ok().json(predictions)
}

#[derive(Deserialize, Serialize)]
pub struct SettingsPayload {
    pub ai_model: Option<String>,
    pub watch_pairs: Option<String>,
    pub analysis_interval_secs: Option<u64>,
}

#[derive(Serialize)]
struct SettingsResponse {
    ai_model: String,
    ai_url: String,
    watch_pairs: String,
    analysis_interval_secs: String,
    news_source: String,
}

#[get("/api/settings")]
pub async fn get_settings() -> HttpResponse {
    let settings = SettingsResponse {
        ai_model: std::env::var("AI_MODEL")
            .unwrap_or_else(|_| "llama-3.3-70b-versatile".into()),
        ai_url: std::env::var("AI_API_URL")
            .unwrap_or_else(|_| "https://api.groq.com/openai/v1".into()),
        watch_pairs: std::env::var("WATCH_PAIRS")
            .unwrap_or_else(|_| "BTCUSDT,ETHUSDT".into()),
        analysis_interval_secs: std::env::var("ANALYSIS_INTERVAL_SECS")
            .unwrap_or_else(|_| "300".into()),
        news_source: "Free RSS (CoinDesk + CoinTelegraph)".into(),
    };
    HttpResponse::Ok().json(settings)
}

#[post("/api/settings")]
pub async fn update_settings(body: Json<SettingsPayload>) -> HttpResponse {
    if let Some(ref model) = body.ai_model {
        std::env::set_var("AI_MODEL", model);
    }
    if let Some(ref pairs) = body.watch_pairs {
        std::env::set_var("WATCH_PAIRS", pairs);
    }
    if let Some(secs) = body.analysis_interval_secs {
        std::env::set_var("ANALYSIS_INTERVAL_SECS", secs.to_string());
    }

    tracing::info!("Settings updated from frontend");
    HttpResponse::Ok().json(serde_json::json!({"success": true, "message": "Settings updated"}))
}
