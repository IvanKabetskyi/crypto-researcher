use serde::{Deserialize, Serialize};

use crate::domain::market::entities::MarketSnapshot;
use crate::domain::prediction::entities::Prediction;
use crate::domain::prediction::services::AnalysisService;

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    content: Option<Vec<ContentBlock>>,
    error: Option<ChatError>,
}

#[derive(Deserialize, Debug)]
struct ChatError {
    message: String,
}

#[derive(Deserialize, Debug)]
struct ContentBlock {
    text: Option<String>,
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
            .unwrap_or_else(|_| "https://api.anthropic.com/v1".into());
        let model = std::env::var("AI_MODEL")
            .unwrap_or_else(|_| "claude-haiku-4-5-20251001".into());
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
            return Err("AI_API_KEY not set. Get a key at https://console.anthropic.com".into());
        }

        let tickers_json = snapshot.tickers_to_json();
        let klines_json = snapshot.klines_to_json();
        let news_json = snapshot.news_to_json();

        let user_content =
            AnalysisService::build_analysis_prompt(&tickers_json, &klines_json, &news_json, timeframe);

        let system_content = format!(
            "You are a professional cryptocurrency trading analyst. \
            You analyze market data, price action, volume, and news sentiment to provide \
            trading predictions with confidence scores. \
            The prediction horizon/timeframe is: {}. \
            Set target_price and stop_loss appropriate for this timeframe. \
            You MUST respond ONLY with valid JSON - no markdown, no code fences, no extra text. \
            Your response must be a JSON object with a \"predictions\" array. \
            Each prediction object must have these exact keys: \
            \"symbol\" (string like \"BTCUSDT\"), \
            \"direction\" (\"long\" or \"short\"), \
            \"confidence\" (number 0-100), \
            \"reasoning\" (string explanation), \
            \"entry_price\" (number), \
            \"target_price\" (number), \
            \"stop_loss\" (number). \
            Always provide at least one prediction for each symbol in the data. \
            Be realistic with confidence scores - use 70-90 range for strong signals.",
            timeframe,
        );

        let request_body = ChatRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            system: system_content,
            messages: vec![
                ChatMessage {
                    role: String::from("user"),
                    content: user_content,
                },
            ],
        };

        let url = format!("{}/messages", self.base_url);

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

        let chat_response: ChatResponse = match serde_json::from_str(&body) {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Failed to parse AI response: {}. Body: {}", e, &body[..body.len().min(500)]);
                return Err(format!("Failed to parse AI response: {}", e).into());
            }
        };

        if let Some(error) = chat_response.error {
            tracing::error!("AI API error: {}", error.message);
            return Err(format!("AI API error: {}", error.message).into());
        }

        let blocks = chat_response.content.unwrap_or_default();
        let content = blocks
            .iter()
            .filter_map(|b| b.text.as_deref())
            .collect::<Vec<_>>()
            .join("");

        if content.is_empty() {
            return Err("Empty content in AI response".into());
        }

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
