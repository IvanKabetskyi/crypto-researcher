use crate::infrastructure::repositories::prediction::PredictionRepository;

use crate::application::dto::history_dto::HistoryDto;
use crate::application::dto::prediction_dto::PredictionDto;
use crate::application::error::DataError;
use crate::application::request_dto::history_params_dto::HistoryParams;

pub async fn get_history_use_case(
    params: HistoryParams,
) -> Result<HistoryDto, DataError> {
    let prediction_repository = PredictionRepository::new().await;
    let result = prediction_repository.get_history(params).await;

    if result.is_err() {
        return Err(result.err().unwrap());
    }

    let (predictions, total, page, per_page) = result.unwrap();

    let items: Vec<PredictionDto> = predictions
        .into_iter()
        .map(|p| PredictionDto::transform_entity(p))
        .collect();

    let total_pages = if per_page > 0 {
        (total + per_page - 1) / per_page
    } else {
        0
    };

    Ok(HistoryDto {
        items,
        total,
        page,
        per_page,
        total_pages,
    })
}
