use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnalyzeParams {
    pub pairs: Vec<String>,
    pub timeframe: String,
    pub min_confidence: f64,
}
