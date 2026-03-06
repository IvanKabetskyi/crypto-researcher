use serde::{Deserialize, Serialize};

use crate::domain::market::entities::MarketSnapshot;
use crate::domain::prediction::entities::Prediction;
use crate::domain::prediction::services::AnalysisService;

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Serialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct AnthropicResponse {
    content: Option<Vec<ContentBlock>>,
    error: Option<AnthropicError>,
}

#[derive(Deserialize, Debug)]
struct ContentBlock {
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct AnthropicError {
    message: String,
}

#[derive(Deserialize, Debug)]
struct AiPredictionResponse {
    predictions: Option<Vec<AiPrediction>>,
}

#[derive(Deserialize, Debug)]
struct AiPrediction {
    symbol: Option<String>,
    direction: Option<String>,
    confidence: Option<f64>,
    reasoning: Option<String>,
    entry_price: Option<f64>,
    target_price: Option<f64>,
    stop_loss: Option<f64>,
}

pub struct AIService {
    base_url: String,
    model: String,
    api_key: String,
    client: reqwest::Client,
}

impl AIService {
    pub fn new() -> Self {
        let base_url = std::env::var("AI_API_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com".into());
        let model = std::env::var("AI_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".into());
        let api_key = std::env::var("AI_API_KEY")
            .unwrap_or_default();

        Self {
            base_url,
            model,
            api_key,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    pub async fn analyze(
        &self,
        snapshot: &MarketSnapshot,
        timeframe: &str,
    ) -> Result<Vec<Prediction>, Box<dyn std::error::Error + Send + Sync>> {
        if self.api_key.is_empty() {
            return Err("AI_API_KEY not set. Set your Anthropic API key.".into());
        }

        let tickers_json = snapshot.tickers_to_json();
        let klines_json = snapshot.klines_to_json();
        let news_json = snapshot.news_to_json();

        let user_content =
            AnalysisService::build_analysis_prompt(&tickers_json, &klines_json, &news_json, timeframe);

        let system_content = format!(
            "You are a conservative, risk-averse cryptocurrency trading analyst. \
            Your #1 priority is ACCURACY — it is far better to give a low-confidence neutral signal \
            than to predict a direction that turns out wrong.\n\n\
            PREDICTION HORIZON: {timeframe}\n\n\
            STEP-BY-STEP ANALYSIS (do this internally for each symbol before deciding):\n\n\
            1. TREND IDENTIFICATION from candle data:\n\
            - Look at the last 10-20 candles. Are closes consistently rising, falling, or ranging?\n\
            - Identify the dominant trend direction. Do NOT predict against the trend unless there is \
              an extremely strong reversal signal (double bottom/top, massive volume spike at support/resistance).\n\
            - If price is choppy/ranging with no clear direction, set confidence below 50.\n\n\
            2. SUPPORT & RESISTANCE:\n\
            - Identify key levels from recent highs and lows in the candle data.\n\
            - If price is near resistance, be cautious about long. If near support, be cautious about short.\n\
            - Set stop_loss just beyond the nearest support (for long) or resistance (for short).\n\
            - Set target_price at the next logical level, NOT an arbitrary percentage.\n\n\
            3. VOLUME CONFIRMATION:\n\
            - A move with increasing volume is more reliable than one with declining volume.\n\
            - If recent candles show declining volume, reduce confidence by 10-15 points.\n\n\
            4. MOMENTUM & CANDLE PATTERNS:\n\
            - Look for: consecutive green/red candles, wicks (rejection), body size vs wick ratio.\n\
            - Long upper wicks at highs = bearish rejection. Long lower wicks at lows = bullish rejection.\n\n\
            5. NEWS, MACRO & MARKET CONTEXT (IMPORTANT — weigh heavily):\n\
            - READ every news headline carefully. News drives crypto more than technicals.\n\
            - Regulatory news (SEC actions, country bans/approvals, ETF decisions) = MAJOR impact. \
              Negative regulatory news should override bullish technical signals.\n\
            - Exchange hacks, depegs, project failures = strong bearish, reduce confidence for affected coins.\n\
            - Whale movements, large transfers to exchanges = bearish pressure signal.\n\
            - Positive adoption news (institutional buying, partnerships, network upgrades) = bullish catalyst.\n\
            - Fed/macro news (rate decisions, inflation data, banking crisis) affects ALL crypto — \
              factor this into every prediction, not just BTC.\n\
            - Bitcoin dominance shift: if BTC is pumping, altcoins often lag or dump — adjust altcoin \
              predictions accordingly.\n\
            - Consider the overall market sentiment: is the market in fear or greed mode based on \
              the price action and news tone? Fear = more likely to drop further, Greed = more likely to continue up.\n\
            - If news sentiment strongly contradicts the technical setup, side with news and lower confidence.\n\
            - In your reasoning, ALWAYS mention relevant news and how it affects your prediction.\n\n\
            CRITICAL RULES FOR ACCURACY:\n\
            - NEVER predict against the prevailing trend on short timeframes (5min, 30min, 1h) \
              unless you see a clear reversal pattern with volume confirmation.\n\
            - If the 24h change is strongly negative (>3% drop) and candles show continued selling, \
              prefer SHORT or low-confidence signals. Do not call a bottom without evidence.\n\
            - If the 24h change is strongly positive (>3% gain) and candles show continued buying, \
              prefer LONG. Do not call a top without evidence.\n\
            - If price is in a tight range with no momentum, set confidence to 40-55 (weak/neutral).\n\
            - Risk/reward ratio must be at least 1.5:1. If you cannot find a setup with this ratio, \
              set confidence below 50.\n\n\
            TARGET & STOP-LOSS SIZING by timeframe:\n\
            - 5min: target 0.1-0.3% from entry, stop 0.1-0.2%\n\
            - 30min: target 0.2-0.5% from entry, stop 0.15-0.3%\n\
            - 1h: target 0.3-1.0% from entry, stop 0.2-0.5%\n\
            - 6h: target 0.5-2.0% from entry, stop 0.3-1.0%\n\
            - 12h: target 1.0-3.0% from entry, stop 0.5-1.5%\n\
            - 24h: target 1.5-5.0% from entry, stop 1.0-2.5%\n\n\
            REASONING: Write 3-5 sentences. Reference specific prices, candle patterns, \
            volume behavior, and support/resistance levels from the data. \
            Explain what confirms your direction AND what would invalidate it.\n\n\
            OUTPUT: Valid JSON only — no markdown, no code fences.\n\
            {{\"predictions\": [{{\"symbol\": \"BTCUSDT\", \"direction\": \"long\" or \"short\", \
            \"confidence\": 0-100, \"reasoning\": \"detailed\", \
            \"entry_price\": number, \"target_price\": number, \"stop_loss\": number}}]}}\n\
            One prediction per symbol. entry_price = current market price.",
            timeframe = timeframe,
        );

        let request_body = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 8192,
            system: system_content,
            messages: vec![
                AnthropicMessage {
                    role: String::from("user"),
                    content: user_content,
                },
            ],
        };

        let url = format!("{}/v1/messages", self.base_url);

        tracing::info!("Calling AI model: {} at {}", self.model, self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            tracing::error!("AI HTTP {}: {}", status, body);
            return Err(format!("AI returned HTTP {}: {}", status, body).into());
        }

        tracing::debug!("AI raw response: {}", &body[..body.len().min(500)]);

        let anthropic_response: AnthropicResponse = match serde_json::from_str(&body) {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Failed to parse AI response: {}. Body: {}", e, &body[..body.len().min(500)]);
                return Err(format!("Failed to parse AI response: {}", e).into());
            }
        };

        if let Some(error) = anthropic_response.error {
            tracing::error!("AI API error: {}", error.message);
            return Err(format!("AI API error: {}", error.message).into());
        }

        let content_blocks = anthropic_response.content.unwrap_or_default();
        if content_blocks.is_empty() {
            return Err("No content returned from AI".into());
        }

        let content = match &content_blocks[0].text {
            Some(c) => c.clone(),
            None => return Err("Empty text in AI response".into()),
        };

        tracing::info!("AI response content length: {} chars", content.len());

        // Clean markdown fences if AI wrapped JSON in ```json ... ```
        let cleaned = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let ai_response: AiPredictionResponse = match serde_json::from_str(cleaned) {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Failed to parse AI predictions JSON: {}. Content: {}", e, &cleaned[..cleaned.len().min(500)]);
                return Err(format!("Failed to parse AI JSON: {}. Content: {}", e, &cleaned[..cleaned.len().min(300)]).into());
            }
        };

        let raw_predictions = ai_response.predictions.unwrap_or_default();

        let predictions: Vec<Prediction> = raw_predictions
            .iter()
            .filter_map(|p| {
                let symbol = p.symbol.as_deref()?;
                let direction = p.direction.as_deref()?;
                let confidence = p.confidence?;
                let reasoning = p.reasoning.as_deref().unwrap_or("No reasoning provided");
                let entry_price = p.entry_price?;
                let target_price = p.target_price?;
                let stop_loss = p.stop_loss?;

                Some(Prediction::new(
                    symbol,
                    direction,
                    confidence,
                    reasoning,
                    entry_price,
                    target_price,
                    stop_loss,
                    None,
                    None,
                    None,
                    None,
                    Some(timeframe.to_string()),
                ))
            })
            .collect();

        tracing::info!("Parsed {} valid predictions from AI response", predictions.len());

        Ok(predictions)
    }
}
