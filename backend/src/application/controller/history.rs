use actix_web::{get, web::Query, HttpResponse};

use crate::application::dto::history_dto::HistoryDto;
use crate::application::request_dto::history_params_dto::HistoryParams;

use crate::application::usecases::get_history::get_history_use_case;

#[get("/api/predictions/history")]
pub async fn get_history(
    query: Query<HistoryParams>,
) -> HttpResponse {
    let params = query.into_inner();

    match get_history_use_case(params).await {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(e) => {
            tracing::warn!("Failed to get prediction history: {}", e.message);
            HttpResponse::Ok().json(HistoryDto::empty())
        }
    }
}
