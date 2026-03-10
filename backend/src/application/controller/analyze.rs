use actix_web::{post, web::{Json, Bytes}, HttpResponse};
use tokio::sync::mpsc;

use crate::application::request_dto::analyze_params_dto::AnalyzeParams;
use crate::application::usecases::run_analysis::run_analysis_use_case;

#[post("/api/analyze")]
pub async fn analyze_stream(
    body: Json<AnalyzeParams>,
) -> HttpResponse {
    let params = body.into_inner();
    tracing::info!("SSE analysis started: pairs={:?}, timeframe={}", params.pairs, params.timeframe);

    let (tx, rx) = mpsc::unbounded_channel::<String>();

    tokio::spawn(async move {
        let (progress_tx, mut progress_rx) = mpsc::unbounded_channel::<String>();
        let tx_for_progress = tx.clone();
        let tx_for_keepalive = tx.clone();

        // Keepalive every 15s to prevent proxy/gateway timeout
        let keepalive_task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                if tx_for_keepalive.send(": keepalive\n\n".to_string()).is_err() {
                    break;
                }
            }
        });

        // Forward progress stages as SSE events
        let progress_task = tokio::spawn(async move {
            while let Some(stage) = progress_rx.recv().await {
                let sse = format!("event: stage\ndata: {}\n\n", stage);
                if tx_for_progress.send(sse).is_err() {
                    break;
                }
            }
        });

        match run_analysis_use_case(params, Some(progress_tx)).await {
            Ok(predictions) => {
                let _ = progress_task.await;
                keepalive_task.abort();
                tracing::info!("SSE analysis completed with {} predictions", predictions.len());
                let json = serde_json::to_string(&predictions).unwrap_or_default();
                let _ = tx.send(format!("event: result\ndata: {}\n\n", json));
            }
            Err(e) => {
                let _ = progress_task.await;
                keepalive_task.abort();
                tracing::error!("SSE analysis failed: {}", e);
                let _ = tx.send(format!("event: error\ndata: {}\n\n", e));
            }
        }
        // tx drops here → stream closes
    });

    let stream = futures::stream::unfold(rx, |mut rx| async {
        match rx.recv().await {
            Some(msg) => Some((Ok::<Bytes, actix_web::Error>(Bytes::from(msg)), rx)),
            None => None,
        }
    });

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("Connection", "keep-alive"))
        .streaming(stream)
}
