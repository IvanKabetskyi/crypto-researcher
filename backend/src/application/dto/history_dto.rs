use serde::Serialize;

use crate::application::dto::prediction_dto::PredictionDto;

#[derive(Debug, Serialize)]
pub struct HistoryDto {
    pub items: Vec<PredictionDto>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl HistoryDto {
    pub fn empty() -> Self {
        Self {
            items: vec![],
            total: 0,
            page: 1,
            per_page: 20,
            total_pages: 0,
        }
    }
}
