use serde::Deserialize;

use crate::domain::market::entities::{DerivativesData, Kline, Ticker};

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

#[derive(Deserialize)]
struct BybitOrderbookResult {
    a: Vec<Vec<String>>, // asks [price, qty]
    b: Vec<Vec<String>>, // bids [price, qty]
}

#[derive(Deserialize)]
struct BybitOrderbookResponse {
    #[serde(rename = "retCode")]
    ret_code: i32,
    result: BybitOrderbookResult,
}

#[derive(Deserialize)]
struct BybitFundingItem {
    #[serde(rename = "fundingRate")]
    funding_rate: String,
}

#[derive(Deserialize)]
struct BybitOpenInterestItem {
    #[serde(rename = "openInterest")]
    open_interest: String,
}

#[derive(Deserialize)]
struct BybitLongShortItem {
    #[serde(rename = "buyRatio")]
    buy_ratio: String,
    #[serde(rename = "sellRatio")]
    sell_ratio: String,
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

    pub async fn fetch_derivatives_data(
        &self,
        symbol: &str,
    ) -> Result<DerivativesData, Box<dyn std::error::Error + Send + Sync>> {
        let (orderbook_ratio, bid_volume, ask_volume) = self.fetch_orderbook(symbol).await
            .unwrap_or((1.0, 0.0, 0.0));

        let funding_rate = self.fetch_funding_rate(symbol).await.unwrap_or(0.0);
        let open_interest = self.fetch_open_interest(symbol).await.unwrap_or(0.0);
        let (long_ratio, short_ratio) = self.fetch_long_short_ratio(symbol).await
            .unwrap_or((0.5, 0.5));

        Ok(DerivativesData::new(
            symbol,
            orderbook_ratio,
            bid_volume,
            ask_volume,
            funding_rate,
            open_interest,
            long_ratio,
            short_ratio,
        ))
    }

    async fn fetch_orderbook(
        &self,
        symbol: &str,
    ) -> Result<(f64, f64, f64), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/v5/market/orderbook?category=linear&symbol={}&limit=50",
            self.base_url, symbol
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let parsed: BybitOrderbookResponse = serde_json::from_str(&body)?;

        if parsed.ret_code != 0 {
            return Err(format!("Orderbook error for {}", symbol).into());
        }

        let bid_volume: f64 = parsed.result.b.iter()
            .filter_map(|level| level.get(1)?.parse::<f64>().ok())
            .sum();
        let ask_volume: f64 = parsed.result.a.iter()
            .filter_map(|level| level.get(1)?.parse::<f64>().ok())
            .sum();

        let ratio = if ask_volume > 0.0 { bid_volume / ask_volume } else { 1.0 };

        Ok((ratio, bid_volume, ask_volume))
    }

    async fn fetch_funding_rate(
        &self,
        symbol: &str,
    ) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/v5/market/funding/history?category=linear&symbol={}&limit=1",
            self.base_url, symbol
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let parsed: BybitResponse = serde_json::from_str(&body)?;

        if parsed.ret_code != 0 {
            return Err(format!("Funding rate error for {}", symbol).into());
        }

        let items: Vec<BybitFundingItem> = serde_json::from_value(parsed.result.list)?;
        let rate = items.first()
            .map(|i| i.funding_rate.parse::<f64>().unwrap_or(0.0))
            .unwrap_or(0.0);

        Ok(rate)
    }

    async fn fetch_open_interest(
        &self,
        symbol: &str,
    ) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/v5/market/open-interest?category=linear&symbol={}&intervalTime=1h&limit=1",
            self.base_url, symbol
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let parsed: BybitResponse = serde_json::from_str(&body)?;

        if parsed.ret_code != 0 {
            return Err(format!("Open interest error for {}", symbol).into());
        }

        let items: Vec<BybitOpenInterestItem> = serde_json::from_value(parsed.result.list)?;
        let oi = items.first()
            .map(|i| i.open_interest.parse::<f64>().unwrap_or(0.0))
            .unwrap_or(0.0);

        Ok(oi)
    }

    async fn fetch_long_short_ratio(
        &self,
        symbol: &str,
    ) -> Result<(f64, f64), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/v5/market/account-ratio?category=linear&symbol={}&period=1h&limit=1",
            self.base_url, symbol
        );

        let response = self.client.get(&url).send().await?;
        let body = response.text().await?;
        let parsed: BybitResponse = serde_json::from_str(&body)?;

        if parsed.ret_code != 0 {
            return Err(format!("Long/short ratio error for {}", symbol).into());
        }

        let items: Vec<BybitLongShortItem> = serde_json::from_value(parsed.result.list)?;
        let (buy, sell) = items.first()
            .map(|i| (
                i.buy_ratio.parse::<f64>().unwrap_or(0.5),
                i.sell_ratio.parse::<f64>().unwrap_or(0.5),
            ))
            .unwrap_or((0.5, 0.5));

        Ok((buy, sell))
    }
}
