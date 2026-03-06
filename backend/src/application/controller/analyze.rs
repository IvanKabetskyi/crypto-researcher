use actix_web::{post, web::Json, HttpResponse};

use crate::application::request_dto::analyze_params_dto::AnalyzeParams;
use crate::application::usecases::run_analysis::run_analysis_use_case;

#[post("/api/analyze")]
pub async fn trigger_analysis(body: Json<AnalyzeParams>) -> HttpResponse {
    let params = body.into_inner();
    tracing::info!("Manual analysis triggered from frontend: pairs={:?}, timeframe={}", params.pairs, params.timeframe);
    match run_analysis_use_case(params).await {
        Ok(predictions) => HttpResponse::Ok().json(predictions),
        Err(e) => {
            tracing::error!("Analysis failed: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Analysis failed: {}", e)
            }))
        }
    }
}
