use serde::Deserialize;

#[derive(Deserialize)]
pub struct FilterParams {
    pub symbol: Option<String>,
    pub min_confidence: Option<f64>,
    pub direction: Option<String>,
    pub limit: Option<i64>,
}
