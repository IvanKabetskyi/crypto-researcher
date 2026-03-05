use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TimeframeDto {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigDto {
    pub pairs: Vec<String>,
    pub timeframes: Vec<TimeframeDto>,
}

impl ConfigDto {
    pub fn default_config() -> Self {
        Self {
            pairs: vec![
                "BTCUSDT".into(),
                "ETHUSDT".into(),
                "SOLUSDT".into(),
                "BNBUSDT".into(),
                "XRPUSDT".into(),
                "DOGEUSDT".into(),
                "ADAUSDT".into(),
                "AVAXUSDT".into(),
            ],
            timeframes: vec![
                TimeframeDto { value: "5min".into(), label: "5 min".into() },
                TimeframeDto { value: "30min".into(), label: "30 min".into() },
                TimeframeDto { value: "1h".into(), label: "1h".into() },
                TimeframeDto { value: "6h".into(), label: "6h".into() },
                TimeframeDto { value: "12h".into(), label: "12h".into() },
                TimeframeDto { value: "24h".into(), label: "24h".into() },
            ],
        }
    }
}
