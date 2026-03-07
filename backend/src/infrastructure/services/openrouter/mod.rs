use serde::{Deserialize, Serialize};

use crate::domain::market::entities::MarketSnapshot;
use crate::domain::prediction::entities::Prediction;
use crate::domain::prediction::services::AnalysisService;

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    temperature: f32,
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

#[derive(Deserialize, Debug, Clone)]
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
    confirmation_model: String,
    api_key: String,
    client: reqwest::Client,
}

impl AIService {
    pub fn new() -> Self {
        let base_url = std::env::var("AI_API_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com".into());
        let model = std::env::var("AI_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".into());
        let confirmation_model = std::env::var("AI_CONFIRMATION_MODEL")
            .unwrap_or_else(|_| "claude-haiku-4-5-20251001".into());
        let api_key = std::env::var("AI_API_KEY")
            .unwrap_or_default();

        Self {
            base_url,
            model,
            confirmation_model,
            api_key,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(180))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    async fn call_model(
        &self,
        model: &str,
        system: &str,
        user_content: &str,
        max_tokens: u32,
        prefill: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut messages = vec![AnthropicMessage {
            role: "user".into(),
            content: user_content.to_string(),
        }];

        if let Some(prefix) = prefill {
            messages.push(AnthropicMessage {
                role: "assistant".into(),
                content: prefix.to_string(),
            });
        }

        let request_body = AnthropicRequest {
            model: model.to_string(),
            max_tokens,
            temperature: 0.0,
            system: system.to_string(),
            messages,
        };

        let url = format!("{}/v1/messages", self.base_url);

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
            tracing::error!("{} HTTP {}: {}", model, status, body);
            return Err(format!("{} returned HTTP {}: {}", model, status, body).into());
        }

        let anthropic_response: AnthropicResponse = serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse {} response: {}", model, e))?;

        if let Some(error) = anthropic_response.error {
            return Err(format!("{} API error: {}", model, error.message).into());
        }

        let content_blocks = anthropic_response.content.unwrap_or_default();
        let text = content_blocks
            .first()
            .and_then(|b| b.text.clone())
            .ok_or_else(|| format!("Empty response from {}", model))?;

        Ok(text)
    }

    fn parse_predictions(raw: &str) -> Result<Vec<AiPrediction>, Box<dyn std::error::Error + Send + Sync>> {
        let cleaned = raw
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        // The assistant message is prefilled with {"predictions":[ so the response
        // continues from there. Prepend the prefix back to form valid JSON.
        let full_json = if cleaned.starts_with('{') || cleaned.starts_with("[{") {
            // If it already looks like full JSON or the model repeated the prefix
            if cleaned.starts_with("{\"predictions\"") {
                cleaned.to_string()
            } else {
                format!("{{\"predictions\":[{}", cleaned)
            }
        } else {
            cleaned.to_string()
        };

        let response: AiPredictionResponse = serde_json::from_str(&full_json)
            .map_err(|e| format!("JSON parse error: {}. Content: {}", e, &full_json[..full_json.len().min(300)]))?;

        Ok(response.predictions.unwrap_or_default())
    }

    fn build_technical_analysis_prompt(timeframe: &str) -> String {
        format!(
            "You are a professional cryptocurrency technical analyst. \
            You receive pre-computed technical indicators and raw candle data.\n\n\
            Perform a DETAILED technical analysis for each symbol on the {timeframe} timeframe.\n\n\
            For EACH symbol, analyze in this exact order:\n\n\
            1. TREND: What does the SMA crossover (sma_trend) say? Is price above or below both SMAs? \
            How many green vs red candles? Is this a clear trend or ranging?\n\n\
            2. RSI: Is it overbought (>70), oversold (<30), or neutral? \
            Does RSI agree or diverge from the trend?\n\n\
            3. EXHAUSTION CHECK (CRITICAL): \
            Look at consecutive_streak — how many candles in a row went the same direction? \
            Look at exhaustion_signal — does it say exhaustion or overextension? \
            Look at dist_from_sma20 — is price stretched too far from the mean? \
            If ANY exhaustion signal is present, the current trend is likely ENDING. \
            After a prolonged move up, expect a move DOWN. After a prolonged move down, expect a move UP.\n\n\
            4. MOMENTUM: Is the move still fresh (small momentum %) or already extended (large %)? \
            A move that already happened is NOT a signal to enter — it's a signal the move may be done.\n\n\
            5. VOLUME: Does volume_ratio confirm the current move (>1.3) or show weakness (<0.7)?\n\n\
            6. CANDLE PATTERN: What does last_candle_signal say? \
            Rejection wicks are strong reversal signals.\n\n\
            7. SUPPORT/RESISTANCE: Where is price relative to key levels? \
            Is it near support (potential long) or resistance (potential short)?\n\n\
            8. NEWS: Any headlines that override the technical picture?\n\n\
            9. CONCLUSION for each symbol: Based on ALL the above, what is the most likely direction \
            for the NEXT {timeframe} candle(s)? Is this a trend-following or mean-reversion setup? \
            How confident are you (low/medium/high)?\n\n\
            Write your analysis as plain text. Be specific — reference actual indicator values.",
            timeframe = timeframe,
        )
    }

    fn build_prediction_prompt(technical_analysis: &str, timeframe: &str) -> String {
        format!(
            "You are a cryptocurrency trading analyst. Below is a detailed technical analysis \
            that was just performed on the market data.\n\n\
            === TECHNICAL ANALYSIS ===\n{technical_analysis}\n\n\
            Based on this analysis, generate trading predictions for the {timeframe} timeframe.\n\n\
            RULES:\n\
            - Your predictions MUST align with the analysis conclusions above\n\
            - If the analysis found exhaustion/reversal signals, predict the REVERSAL direction\n\
            - If the analysis found a fresh trend, predict trend continuation\n\
            - entry_price = current market price from the data\n\
            - For SHORT: target BELOW entry, stop_loss ABOVE entry\n\
            - For LONG: target ABOVE entry, stop_loss BELOW entry\n\
            - Risk/reward must be at least 1.5:1\n\n\
            CONFIDENCE based on analysis strength:\n\
            - 30-45: Analysis was uncertain or mixed signals\n\
            - 45-60: Moderate clarity, 2-3 indicators aligned\n\
            - 60-75: Strong clarity, most indicators + exhaustion/trend aligned\n\
            - 75-85: Very strong, everything aligned + news confirmed\n\n\
            OUTPUT: Valid JSON only — no markdown, no code fences.\n\
            {{\"predictions\": [{{\"symbol\": \"BTCUSDT\", \"direction\": \"long\" or \"short\", \
            \"confidence\": 0-100, \"reasoning\": \"summarize the key analysis points\", \
            \"entry_price\": number, \"target_price\": number, \"stop_loss\": number}}]}}\n\
            One prediction per symbol.",
            technical_analysis = technical_analysis,
            timeframe = timeframe,
        )
    }

    fn build_confirmation_prompt(sonnet_predictions_json: &str, timeframe: &str) -> String {
        format!(
            "You are an independent cryptocurrency trading analyst performing a SECOND OPINION review.\n\n\
            Another analyst produced the predictions below for the {timeframe} timeframe. \
            You have access to the same market data.\n\n\
            YOUR TASK:\n\
            - Independently analyze the market data provided.\n\
            - For each prediction, decide if you AGREE or DISAGREE with the direction.\n\
            - Provide your own confidence score (0-100) based on YOUR analysis.\n\
            - If you disagree on direction, set your confidence for the OPPOSITE direction.\n\
            - Write a brief reasoning (1-2 sentences) explaining your view.\n\n\
            PREVIOUS ANALYST'S PREDICTIONS:\n{sonnet_predictions_json}\n\n\
            OUTPUT: Valid JSON only — no markdown, no code fences.\n\
            {{\"predictions\": [{{\"symbol\": \"BTCUSDT\", \"direction\": \"long\" or \"short\", \
            \"confidence\": 0-100, \"reasoning\": \"your independent assessment\"}}]}}\n\
            One prediction per symbol.",
            timeframe = timeframe,
            sonnet_predictions_json = sonnet_predictions_json,
        )
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
        let indicators_json = snapshot.compute_indicators(timeframe);

        let user_content =
            AnalysisService::build_analysis_prompt(&tickers_json, &klines_json, &news_json, &indicators_json, timeframe);

        // Step 1: Technical analysis (free text — AI thinks through the data first)
        let ta_system = Self::build_technical_analysis_prompt(timeframe);
        tracing::info!("Step 1: Technical analysis with {}", self.model);
        let technical_analysis = self
            .call_model(&self.model, &ta_system, &user_content, 4096, None)
            .await?;
        tracing::info!("Technical analysis: {} chars", technical_analysis.len());

        // Step 2: Generate predictions based on the analysis
        let prediction_system = Self::build_prediction_prompt(&technical_analysis, timeframe);
        tracing::info!("Step 2: Generating predictions with {}", self.model);
        let sonnet_raw = self
            .call_model(&self.model, &prediction_system, &user_content, 4096, Some("{\"predictions\":["))
            .await?;
        tracing::info!("Sonnet predictions: {} chars", sonnet_raw.len());

        let sonnet_predictions = Self::parse_predictions(&sonnet_raw)?;
        if sonnet_predictions.is_empty() {
            return Err("Sonnet returned 0 predictions".into());
        }
        tracing::info!("Sonnet returned {} predictions", sonnet_predictions.len());

        // Step 3: Confirmation with Haiku
        let sonnet_summary: Vec<serde_json::Value> = sonnet_predictions.iter().map(|p| {
            serde_json::json!({
                "symbol": p.symbol,
                "direction": p.direction,
                "confidence": p.confidence,
                "entry_price": p.entry_price,
                "target_price": p.target_price,
                "stop_loss": p.stop_loss,
                "reasoning": p.reasoning,
            })
        }).collect();
        let sonnet_json = serde_json::to_string(&sonnet_summary).unwrap_or_default();

        let confirmation_system = Self::build_confirmation_prompt(&sonnet_json, timeframe);

        tracing::info!("Step 3: Confirmation with {}", self.confirmation_model);
        let haiku_result = self.call_model(
            &self.confirmation_model,
            &confirmation_system,
            &user_content,
            4096,
            Some("{\"predictions\":["),
        ).await;

        let haiku_predictions = match haiku_result {
            Ok(raw) => {
                tracing::info!("Haiku response: {} chars", raw.len());
                Self::parse_predictions(&raw).unwrap_or_default()
            }
            Err(e) => {
                tracing::warn!("Haiku confirmation failed (using Sonnet only): {}", e);
                vec![]
            }
        };

        // Step 3: Merge — adjust confidence based on agreement
        let predictions: Vec<Prediction> = sonnet_predictions
            .iter()
            .filter_map(|sp| {
                let symbol = sp.symbol.as_deref()?;
                let sonnet_dir = sp.direction.as_deref()?;
                let sonnet_conf = sp.confidence?;
                let reasoning = sp.reasoning.as_deref().unwrap_or("No reasoning provided");
                let entry_price = sp.entry_price?;
                let target_price = sp.target_price?;
                let stop_loss = sp.stop_loss?;

                // Find Haiku's opinion on the same symbol
                let haiku_match = haiku_predictions.iter().find(|hp| {
                    hp.symbol.as_deref() == Some(symbol)
                });

                // Validate target/stop_loss direction
                let (target_price, stop_loss) = if sonnet_dir == "short" && target_price > entry_price {
                    // SHORT: target must be below entry, stop above
                    tracing::warn!("{}: Correcting SHORT target/stop — target was above entry", symbol);
                    (stop_loss.min(entry_price * 0.995), target_price.max(entry_price * 1.005))
                } else if sonnet_dir == "long" && target_price < entry_price {
                    // LONG: target must be above entry, stop below
                    tracing::warn!("{}: Correcting LONG target/stop — target was below entry", symbol);
                    (stop_loss.max(entry_price * 1.005), target_price.min(entry_price * 0.995))
                } else {
                    (target_price, stop_loss)
                };

                let (final_confidence, final_reasoning) = match haiku_match {
                    Some(hp) => {
                        let haiku_dir = hp.direction.as_deref().unwrap_or("");
                        let haiku_conf = hp.confidence.unwrap_or(50.0);
                        let haiku_reasoning = hp.reasoning.as_deref().unwrap_or("");

                        if haiku_dir != sonnet_dir {
                            // Models DISAGREE on direction — skip entirely
                            tracing::info!(
                                "{}: SKIPPED — Sonnet says {}({:.0}%) but Haiku says {}({:.0}%)",
                                symbol, sonnet_dir, sonnet_conf, haiku_dir, haiku_conf
                            );
                            return None;
                        }

                        // Both AGREE — use average confidence
                        let merged = (sonnet_conf + haiku_conf) / 2.0;
                        let combined = format!(
                            "{}\n\n[Confirmed by second analysis ({:.0}%): {}]",
                            reasoning, haiku_conf, haiku_reasoning
                        );
                        tracing::info!(
                            "{}: AGREE ({}) Sonnet {:.0}% + Haiku {:.0}% → avg {:.0}%",
                            symbol, sonnet_dir, sonnet_conf, haiku_conf, merged
                        );
                        (merged, combined)
                    }
                    None => {
                        // Haiku didn't return opinion — use Sonnet as-is
                        tracing::info!("{}: Haiku no opinion, using Sonnet {:.0}%", symbol, sonnet_conf);
                        (sonnet_conf, reasoning.to_string())
                    }
                };

                Some(Prediction::new(
                    symbol,
                    sonnet_dir,
                    final_confidence,
                    &final_reasoning,
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

        tracing::info!("Final merged predictions: {}", predictions.len());

        Ok(predictions)
    }
}
