use crate::application::dto::market_dto::MarketDto;
use crate::application::error::DataError;
use crate::application::mapper::transform_ticker_entity_to_dto::transform_ticker_entity_to_dto;
use crate::infrastructure::services::bybit::BybitService;

pub async fn get_market_use_case() -> Result<Vec<MarketDto>, DataError> {
    let pairs = std::env::var("WATCH_PAIRS").unwrap_or_else(|_| "BTCUSDT,ETHUSDT".into());
    let symbols: Vec<String> = pairs.split(',').map(|s| s.to_string()).collect();

    let bybit_service = BybitService::new();
    let tickers_response = bybit_service.fetch_tickers(&symbols).await;

    if tickers_response.is_err() {
        return Err(DataError::new("failed to fetch market data from Bybit"));
    }

    let tickers = tickers_response.unwrap();

    let dtos: Vec<MarketDto> = tickers
        .iter()
        .map(|t| transform_ticker_entity_to_dto(t))
        .collect();

    Ok(dtos)
}
