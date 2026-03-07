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
                .timeout(std::time::Duration::from_secs(120))
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
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let request_body = AnthropicRequest {
            model: model.to_string(),
            max_tokens,
            temperature: 0.0,
            system: system.to_string(),
            messages: vec![
                AnthropicMessage {
                    role: "user".into(),
                    content: user_content.to_string(),
                },
                AnthropicMessage {
                    role: "assistant".into(),
                    content: "{\"predictions\":[".to_string(),
                },
            ],
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

    fn build_system_prompt(timeframe: &str) -> String {
        format!(
            "You are an ultra-conservative cryptocurrency trading analyst. \
            Your #1 priority is ACCURACY. You would rather miss 10 good trades than take 1 bad trade. \
            Most of the time, the correct answer is LOW CONFIDENCE because markets are uncertain.\n\n\
            PREDICTION HORIZON: {timeframe}\n\n\
            DEFAULT STANCE: Start every analysis at 40% confidence (uncertain). \
            Only INCREASE confidence if you find strong, specific evidence. \
            Most predictions should be 40-60% unless you see exceptional confluence.\n\n\
            STEP-BY-STEP ANALYSIS (do this for each symbol):\n\n\
            1. TREND from candle data:\n\
            - Count the last 20 candles: how many are green vs red?\n\
            - Are closes making higher highs and higher lows (uptrend) or lower highs and lower lows (downtrend)?\n\
            - If roughly equal green/red with no clear pattern = RANGING → confidence stays 35-45.\n\
            - Only a clear trend with 65%+ candles in one direction adds confidence (+10-15).\n\n\
            2. MOMENTUM — IS THE MOVE ALREADY DONE?\n\
            - CRITICAL: If price already moved >2% in one direction in the last few hours, \
              do NOT predict continuation. The move may be exhausted.\n\
            - After a big move (>3%), expect consolidation or reversal, not continuation.\n\
            - Look at the LAST 3-5 candles specifically: is momentum accelerating or fading?\n\
            - Fading momentum (smaller bodies, longer wicks) = reduce confidence by 15-20.\n\n\
            3. SUPPORT & RESISTANCE:\n\
            - Find the highest high and lowest low in the candle data.\n\
            - If price is in the middle of this range = no edge → confidence 35-45.\n\
            - Only if price is bouncing OFF support (for long) or rejecting AT resistance (for short) \
              with volume confirmation should confidence be above 60.\n\
            - Set stop_loss TIGHT: just beyond the nearest candle wick, not at arbitrary levels.\n\n\
            4. VOLUME — THE TRUTH DETECTOR:\n\
            - Compare last 5 candles volume to previous 10 candles average.\n\
            - Declining volume on a move = FAKE MOVE, likely to reverse → confidence -20.\n\
            - Volume spike at a key level = REAL → confidence +10.\n\
            - No volume change = no conviction → keep confidence low.\n\n\
            5. NEWS, MACRO & MARKET CONTEXT:\n\
            - READ every news headline. News is the #1 driver of crypto.\n\
            - Regulatory news (SEC, bans, ETFs) = OVERRIDE all technicals.\n\
            - Exchange hacks, depegs, failures = strong bearish.\n\
            - Whale movements to exchanges = selling pressure.\n\
            - Fed/rate/inflation news affects ALL crypto.\n\
            - If BTC is dumping, do NOT go long on altcoins regardless of their chart.\n\
            - Fear sentiment + downtrend = more downside likely.\n\
            - If no significant news = no catalyst to move → confidence stays low.\n\
            - ALWAYS mention news impact in reasoning.\n\n\
            ANTI-ERROR RULES (these prevent the most common mistakes):\n\
            - DO NOT CHASE: If the 24h change is >3% in either direction, the easy money is gone. \
              Set confidence to 40-50 max unless you see a FRESH reversal setup.\n\
            - DO NOT FIGHT THE TREND: On timeframes ≤1h, NEVER predict against the clear trend.\n\
            - DO NOT PREDICT BOUNCES from round numbers unless there is visible buying/selling volume at that level.\n\
            - DO NOT give >70% confidence unless you have: trend + volume + news all aligned.\n\
            - WHEN IN DOUBT, default to 40-50% confidence. Being uncertain IS the correct answer most of the time.\n\
            - After 3+ consecutive candles in one direction, expect a pullback, not continuation.\n\
            - If the last candle has a very long wick, the NEXT candle often goes the opposite direction.\n\n\
            TARGET & STOP-LOSS (TIGHT — this is critical for accuracy):\n\
            - 5min: target 0.1-0.2%, stop 0.1-0.15%\n\
            - 30min: target 0.15-0.4%, stop 0.1-0.25%\n\
            - 1h: target 0.2-0.6%, stop 0.15-0.35%\n\
            - 6h: target 0.4-1.2%, stop 0.25-0.7%\n\
            - 12h: target 0.8-2.0%, stop 0.4-1.0%\n\
            - 24h: target 1.0-3.0%, stop 0.6-1.5%\n\
            Risk/reward MUST be at least 1.5:1. If you can't find it, set confidence below 45.\n\n\
            REASONING: 3-5 sentences. Reference SPECIFIC prices and candle patterns. \
            State what evidence raised your confidence above 40% baseline. \
            State what would invalidate the trade.\n\n\
            OUTPUT: Valid JSON only — no markdown, no code fences.\n\
            {{\"predictions\": [{{\"symbol\": \"BTCUSDT\", \"direction\": \"long\" or \"short\", \
            \"confidence\": 0-100, \"reasoning\": \"detailed\", \
            \"entry_price\": number, \"target_price\": number, \"stop_loss\": number}}]}}\n\
            One prediction per symbol. entry_price = current market price.",
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

        let user_content =
            AnalysisService::build_analysis_prompt(&tickers_json, &klines_json, &news_json, timeframe);

        let system_prompt = Self::build_system_prompt(timeframe);

        // Step 1: Primary analysis with Sonnet
        tracing::info!("Step 1: Calling primary model {} for analysis", self.model);
        let sonnet_raw = self.call_model(&self.model, &system_prompt, &user_content, 8192).await?;
        tracing::info!("Sonnet response: {} chars", sonnet_raw.len());

        let sonnet_predictions = Self::parse_predictions(&sonnet_raw)?;
        if sonnet_predictions.is_empty() {
            return Err("Sonnet returned 0 predictions".into());
        }
        tracing::info!("Sonnet returned {} predictions", sonnet_predictions.len());

        // Step 2: Confirmation with Haiku
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

        tracing::info!("Step 2: Calling confirmation model {} for review", self.confirmation_model);
        let haiku_result = self.call_model(
            &self.confirmation_model,
            &confirmation_system,
            &user_content,
            4096,
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

                let (final_confidence, final_reasoning) = match haiku_match {
                    Some(hp) => {
                        let haiku_dir = hp.direction.as_deref().unwrap_or("");
                        let haiku_conf = hp.confidence.unwrap_or(50.0);
                        let haiku_reasoning = hp.reasoning.as_deref().unwrap_or("");

                        if haiku_dir == sonnet_dir {
                            // Both agree — use the LOWER confidence (conservative)
                            let merged = sonnet_conf.min(haiku_conf);
                            let combined = format!(
                                "{}\n\n[Confirmed by second analysis: {} | confidence {:.0}%]",
                                reasoning, haiku_reasoning, haiku_conf
                            );
                            tracing::info!("{}: AGREE ({}) Sonnet {:.0}% + Haiku {:.0}% → {:.0}%",
                                symbol, sonnet_dir, sonnet_conf, haiku_conf, merged);
                            (merged, combined)
                        } else {
                            // Disagree on direction — HARD penalty, cap at 40%
                            let merged = (sonnet_conf * 0.4).min(40.0);
                            let combined = format!(
                                "{}\n\n[WARNING: second analysis DISAGREES — suggests {} with {:.0}% confidence: {}]",
                                reasoning, haiku_dir, haiku_conf, haiku_reasoning
                            );
                            tracing::info!("{}: DISAGREE Sonnet {}({:.0}%) vs Haiku {}({:.0}%) → {:.0}%",
                                symbol, sonnet_dir, sonnet_conf, haiku_dir, haiku_conf, merged);
                            (merged, combined)
                        }
                    }
                    None => {
                        // Haiku didn't return a prediction for this symbol — use Sonnet as-is
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
