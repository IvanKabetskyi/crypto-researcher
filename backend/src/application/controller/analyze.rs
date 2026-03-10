use actix_web::{post, get, web::{Json, Path, Data}, HttpResponse};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::application::dto::prediction_dto::PredictionDto;
use crate::application::request_dto::analyze_params_dto::AnalyzeParams;
use crate::application::usecases::run_analysis::run_analysis_use_case;

#[derive(Clone, Serialize)]
pub struct JobStatus {
    pub id: String,
    pub status: String, // "running", "completed", "failed"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictions: Option<Vec<PredictionDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub type JobStore = Arc<RwLock<HashMap<String, JobStatus>>>;

pub fn create_job_store() -> JobStore {
    Arc::new(RwLock::new(HashMap::new()))
}

#[post("/api/analyze")]
pub async fn trigger_analysis(
    body: Json<AnalyzeParams>,
    store: Data<JobStore>,
) -> HttpResponse {
    let params = body.into_inner();
    let job_id = Uuid::new_v4().to_string();

    tracing::info!("Analysis job {} started: pairs={:?}, timeframe={}", job_id, params.pairs, params.timeframe);

    // Insert running job
    {
        let mut jobs = store.write().await;
        jobs.insert(job_id.clone(), JobStatus {
            id: job_id.clone(),
            status: "running".into(),
            predictions: None,
            error: None,
        });
    }

    // Spawn background task
    let store_clone = store.get_ref().clone();
    let jid = job_id.clone();
    tokio::spawn(async move {
        match run_analysis_use_case(params).await {
            Ok(predictions) => {
                tracing::info!("Job {} completed with {} predictions", jid, predictions.len());
                let mut jobs = store_clone.write().await;
                jobs.insert(jid.clone(), JobStatus {
                    id: jid,
                    status: "completed".into(),
                    predictions: Some(predictions),
                    error: None,
                });
            }
            Err(e) => {
                tracing::error!("Job {} failed: {}", jid, e);
                let mut jobs = store_clone.write().await;
                jobs.insert(jid.clone(), JobStatus {
                    id: jid,
                    status: "failed".into(),
                    predictions: None,
                    error: Some(format!("Analysis failed: {}", e)),
                });
            }
        }
    });

    HttpResponse::Accepted().json(serde_json::json!({
        "job_id": job_id,
        "status": "running"
    }))
}

#[get("/api/analyze/{job_id}")]
pub async fn get_analysis_status(
    path: Path<String>,
    store: Data<JobStore>,
) -> HttpResponse {
    let job_id = path.into_inner();
    let jobs = store.read().await;
    match jobs.get(&job_id) {
        Some(job) => HttpResponse::Ok().json(job),
        None => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Job not found"
        })),
    }
}
