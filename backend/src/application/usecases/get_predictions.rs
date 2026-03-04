use crate::infrastructure::repositories::prediction::PredictionRepository;

use crate::application::dto::prediction_dto::PredictionDto;
use crate::application::error::DataError;
use crate::application::mapper::transform_prediction_entity_to_dto::transform_prediction_entity_to_dto;
use crate::application::request_dto::filter_params_dto::FilterParams;

pub async fn get_predictions_use_case(
    filter: FilterParams,
) -> Result<Vec<PredictionDto>, DataError> {
    let prediction_repository = PredictionRepository::new().await;
    let predictions_response = prediction_repository.get_predictions(filter).await;

    if predictions_response.is_err() {
        return Err(predictions_response.err().unwrap());
    }

    let predictions = predictions_response.unwrap();

    let dtos: Vec<PredictionDto> = predictions
        .into_iter()
        .map(|p| transform_prediction_entity_to_dto(p))
        .collect();

    Ok(dtos)
}
