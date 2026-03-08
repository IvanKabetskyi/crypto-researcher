use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnalyzeParams {
    pub pairs: Vec<String>,
    pub timeframe: String,
    pub min_confidence: f64,
    pub bet_value: f64,
}
