use serde::Deserialize;

#[derive(Deserialize)]
pub struct HistoryParams {
    pub symbol: Option<String>,
    pub direction: Option<String>,
    pub outcome: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
