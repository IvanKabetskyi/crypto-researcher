use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TimeframeDto {
    pub value: String,
    pub label: String,
    pub description: String,
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
                TimeframeDto { value: "5min".into(), label: "5 min".into(), description: "Scalping — ultra-short trades lasting 5-30 minutes. Focuses on micro price action and order flow.".into() },
                TimeframeDto { value: "30min".into(), label: "30 min".into(), description: "Intraday — short-term trades lasting 1-4 hours. Captures quick momentum moves within the session.".into() },
                TimeframeDto { value: "1h".into(), label: "1h".into(), description: "Intraday swing — trades lasting 4-12 hours. Balances noise filtering with timely entries.".into() },
                TimeframeDto { value: "6h".into(), label: "6h".into(), description: "Swing — trades lasting 1-3 days. Identifies major trend shifts and key support/resistance breaks.".into() },
                TimeframeDto { value: "12h".into(), label: "12h".into(), description: "Position — trades lasting 3-7 days. Targets multi-day trend continuations with wider stops.".into() },
                TimeframeDto { value: "24h".into(), label: "24h".into(), description: "Macro — trades lasting 1-2 weeks. Captures large directional moves based on daily chart structure.".into() },
            ],
        }
    }
}
