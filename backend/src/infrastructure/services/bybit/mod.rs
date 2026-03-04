use serde::Deserialize;

use crate::domain::market::entities::{Kline, Ticker};

#[derive(Deserialize)]
struct BybitResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,
    result: BybitResult,
}

#[derive(Deserialize)]
struct BybitResult {
    list: serde_json::Value,
}

#[derive(Deserialize)]
struct BybitTickerItem {
    symbol: String,
    #[serde(rename = "lastPrice")]
    last_price: String,
    #[serde(rename = "price24hPcnt")]
    price_24h_pcnt: String,
    #[serde(rename = "volume24h")]
    volume_24h: String,
    #[serde(rename = "highPrice24h")]
    high_price_24h: String,
    #[serde(rename = "lowPrice24h")]
    low_price_24h: String,
}

pub struct BybitService {
    base_url: String,
    client: reqwest::Client,
}

impl BybitService {
    pub fn new() -> Self {
        let base_url =
            std::env::var("BYBIT_API_URL").unwrap_or_else(|_| "https://api.bybit.com".into());

        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_tickers(
        &self,
        symbols: &[String],
    ) -> Result<Vec<Ticker>, Box<dyn std::error::Error + Send + Sync>> {
        let mut tickers: Vec<Ticker> = Vec::new();

        for symbol in symbols {
            let url = format!(
                "{}/v5/market/tickers?category=spot&symbol={}",
                self.base_url, symbol
            );

            let response = self.client.get(&url).send().await?;
            let body = response.text().await?;
            let bybit_response: BybitResponse = serde_json::from_str(&body)?;

            if bybit_response.ret_code != 0 {
                tracing::warn!("Bybit returned non-zero code for {}", symbol);
                continue;
            }

            let items: Vec<BybitTickerItem> =
                serde_json::from_value(bybit_response.result.list)?;

            for item in items {
                let ticker = Ticker::new(
                    &item.symbol,
                    item.last_price.parse::<f64>().unwrap_or(0.0),
                    item.price_24h_pcnt.parse::<f64>().unwrap_or(0.0),
                    item.volume_24h.parse::<f64>().unwrap_or(0.0),
                    item.high_price_24h.parse::<f64>().unwrap_or(0.0),
                    item.low_price_24h.parse::<f64>().unwrap_or(0.0),
                );
                tickers.push(ticker);
            }
        }

        Ok(tickers)
    }

    pub async fn fetch_klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: u32,
    ) -> Result<Vec<Kline>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/v5/market/kline?category=spot&symbol={}&interval={}&limit={}",
            self.base_url, symbol, interval, limit
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let bybit_response: BybitResponse = serde_json::from_str(&body)?;

        if bybit_response.ret_code != 0 {
            return Err(format!("Bybit returned non-zero code for klines {}", symbol).into());
        }

        let raw_list: Vec<Vec<String>> =
            serde_json::from_value(bybit_response.result.list)?;

        let klines: Vec<Kline> = raw_list
            .iter()
            .map(|item| {
                Kline::new(
                    item.get(1)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0),
                    item.get(2)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0),
                    item.get(3)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0),
                    item.get(4)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0),
                    item.get(5)
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0),
                    item.get(0)
                        .and_then(|v| v.parse::<i64>().ok())
                        .unwrap_or(0),
                )
            })
            .collect();

        Ok(klines)
    }
}
