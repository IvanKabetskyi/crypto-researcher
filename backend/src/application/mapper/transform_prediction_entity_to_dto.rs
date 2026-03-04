use crate::domain::prediction::entities::Prediction;

use crate::application::dto::prediction_dto::PredictionDto;

pub fn transform_prediction_entity_to_dto(prediction: Prediction) -> PredictionDto {
    PredictionDto::transform_entity(prediction)
}
