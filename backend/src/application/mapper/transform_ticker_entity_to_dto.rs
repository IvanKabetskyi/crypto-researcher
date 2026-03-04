use crate::domain::market::entities::Ticker;

use crate::application::dto::market_dto::MarketDto;

pub fn transform_ticker_entity_to_dto(ticker: &Ticker) -> MarketDto {
    MarketDto::transform_entity(ticker)
}
