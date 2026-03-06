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
            "You are a senior cryptocurrency trading analyst with deep expertise in technical analysis, \
            on-chain metrics, market microstructure, and sentiment analysis. \
            Your task is to analyze the provided market data and produce actionable trading signals.\n\n\
            ANALYSIS FRAMEWORK — for each symbol, evaluate:\n\
            1. **Price Action & Trend**: Support/resistance levels, trend direction, momentum (from candle data)\n\
            2. **Volume Analysis**: Volume profile, unusual volume spikes, buying vs selling pressure\n\
            3. **Volatility & Risk**: Recent price range, ATR-equivalent, risk/reward ratio\n\
            4. **News & Sentiment**: Impact of recent news, market sentiment shift, catalyst identification\n\
            5. **Cross-Asset Context**: Correlation with BTC if altcoin, broader market conditions\n\n\
            PREDICTION HORIZON: {}\n\
            Set entry_price close to current market price. \
            Set target_price and stop_loss proportional to this timeframe \
            (tighter for short timeframes like 5min-1h, wider for 6h-24h).\n\n\
            REASONING REQUIREMENTS:\n\
            - Provide 2-4 sentences explaining WHY this is a good signal\n\
            - Reference specific data points: price levels, volume changes, news events\n\
            - Explain the risk/reward setup and what would invalidate the trade\n\
            - If confidence is below 60, explain what makes the signal weak\n\n\
            CONFIDENCE SCORING:\n\
            - 80-95: Strong confluence of multiple factors (trend + volume + sentiment aligned)\n\
            - 65-79: Good setup with minor concerns or mixed signals\n\
            - 50-64: Weak or uncertain signal, conflicting indicators\n\
            - Below 50: Avoid — no clear edge\n\n\
            OUTPUT FORMAT: Respond ONLY with valid JSON — no markdown, no code fences, no extra text.\n\
            JSON object with a \"predictions\" array. Each prediction:\n\
            {{\"symbol\": \"BTCUSDT\", \"direction\": \"long\" or \"short\", \"confidence\": 0-100, \
            \"reasoning\": \"detailed explanation\", \"entry_price\": number, \
            \"target_price\": number, \"stop_loss\": number}}\n\
            Provide exactly one prediction per symbol in the data.",
            timeframe,
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
