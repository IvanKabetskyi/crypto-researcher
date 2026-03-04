use serde::Serialize;

use crate::domain::market::entities::Ticker;

#[derive(Debug, Serialize)]
pub struct MarketDto {
    symbol: String,
    price: f64,
    change_24h: f64,
    volume_24h: f64,
}

impl MarketDto {
    pub fn transform_entity(ticker: &Ticker) -> Self {
        Self {
            symbol: ticker.get_symbol(),
            price: ticker.get_last_price(),
            change_24h: ticker.get_price_24h_pcnt(),
            volume_24h: ticker.get_volume_24h(),
        }
    }
}
