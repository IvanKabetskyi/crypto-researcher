use serde::{Deserialize, Serialize};

use crate::domain::market::entities::MarketSnapshot;
use crate::domain::prediction::entities::Prediction;
use crate::domain::prediction::services::AnalysisService;

// ── Anthropic API types ─────────────────────────────────────────────────────

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

// ── Pipeline stage output types ─────────────────────────────────────────────

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct MarketAnalysis {
    symbol: String,
    market_bias: Option<String>,
    trend_strength: Option<String>,
    key_levels: Option<KeyLevels>,
    momentum: Option<String>,
    volume_profile: Option<String>,
    derivatives_sentiment: Option<String>,
    signals: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct KeyLevels {
    support: Option<f64>,
    resistance: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SignalOutput {
    symbol: String,
    decision: Option<String>,
    status: Option<String>,
    confidence: Option<f64>,
    risk_reward: Option<f64>,
    entry_price: Option<f64>,
    target_price: Option<f64>,
    stop_loss: Option<f64>,
    reasoning: Option<Vec<String>>,
    issues: Option<Vec<String>>,
    confluence_score: Option<f64>,
}

#[derive(Deserialize, Debug)]
struct ClassifierResponse {
    signals: Option<Vec<SignalOutput>>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SetupFeatures {
    symbol: String,
    intended_direction: String,
    entry_price: f64,
    target_price: f64,
    stop_loss: f64,
    risk_reward: f64,
    distance_to_resistance_pct: f64,
    distance_to_support_pct: f64,
    volatility_spike: bool,
    leverage_risk: bool,
    liquidation_risk: bool,
    confirmations_count: u32,
    confirmations: Vec<String>,
    market_bias: String,
    trend_strength: String,
    momentum: String,
    volume_profile: String,
    rsi: f64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct RiskAssessment {
    symbol: String,
    decision: Option<String>,
    risk_reward_ratio: Option<f64>,
    position_size_pct: Option<f64>,
    risk_notes: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct StrategyOutput {
    symbol: String,
    execution_action: Option<String>,
    adjusted_entry: Option<f64>,
    adjusted_target: Option<f64>,
    adjusted_stop: Option<f64>,
    adjusted_position_size_pct: Option<f64>,
    execution_notes: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct DetectedIssue {
    source: Option<String>,
    issue: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ReviewResult {
    review_result: Option<ReviewVerdict>,
    detected_issues: Option<Vec<DetectedIssue>>,
    review_notes: Option<Vec<String>>,
    final_approved_plan: Option<ReviewPlan>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ReviewVerdict {
    consistency_status: Option<String>,
    final_verdict: Option<String>,
    final_decision: Option<String>,
    confidence: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ReviewPlan {
    market_bias: Option<String>,
    execution_plan: Option<String>,
    setup_type: Option<String>,
    targets: Option<ReviewTargets>,
    invalidation: Option<f64>,
    risk_decision: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct ReviewTargets {
    primary: Option<f64>,
    secondary: Option<f64>,
}

// ── Wrapper responses for JSON parsing ──────────────────────────────────────

#[derive(Deserialize, Debug)]
struct AnalysisResponse {
    analyses: Option<Vec<MarketAnalysis>>,
}

#[derive(Deserialize, Debug)]
struct RiskResponse {
    assessments: Option<Vec<RiskAssessment>>,
}

#[derive(Deserialize, Debug)]
struct StrategyResponse {
    strategies: Option<Vec<StrategyOutput>>,
}

#[derive(Deserialize, Debug)]
struct ReviewResponse {
    reviews: Option<Vec<ReviewResult>>,
}

// ── AI Service ──────────────────────────────────────────────────────────────

pub struct AIService {
    base_url: String,
    model: String,
    fast_model: String,
    review_model: String,
    api_key: String,
    client: reqwest::Client,
}

impl AIService {
    pub fn new() -> Self {
        let base_url = std::env::var("AI_API_URL")
            .unwrap_or_else(|_| "https://api.anthropic.com".into());
        let model = std::env::var("AI_MODEL")
            .unwrap_or_else(|_| "claude-opus-4-6".into());
        let fast_model = std::env::var("AI_FAST_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".into());
        let review_model = std::env::var("AI_REVIEW_MODEL")
            .unwrap_or_else(|_| "claude-sonnet-4-20250514".into());
        let api_key = std::env::var("AI_API_KEY")
            .unwrap_or_default();

        Self {
            base_url,
            model,
            fast_model,
            review_model,
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
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let messages = vec![AnthropicMessage {
            role: "user".into(),
            content: user_content.to_string(),
        }];

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

    async fn call_model_with_retry(
        &self,
        model: &str,
        system: &str,
        user_content: &str,
        max_tokens: u32,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.call_model(model, system, user_content, max_tokens).await;
        match result {
            Ok(raw) => Ok(raw),
            Err(e) => {
                tracing::warn!("First AI call failed, retrying with repair prompt: {}", e);
                let repair_system = format!(
                    "{}\n\nYour previous response was invalid. Error: {}\n\
                    Return the same result as exactly one valid JSON object.\n\
                    No markdown. No commentary. No trailing text.\n\
                    All strings closed. All braces balanced.",
                    system, e
                );
                self.call_model(model, &repair_system, user_content, max_tokens).await
            }
        }
    }

    /// Extract the first complete JSON value from a string by brace/bracket matching.
    /// Returns the slice up to and including the closing delimiter, ignoring trailing text.
    fn extract_first_json(s: &str) -> &str {
        let mut depth = 0i32;
        let mut in_string = false;
        let mut escape = false;

        for (i, c) in s.char_indices() {
            if escape {
                escape = false;
                continue;
            }
            match c {
                '\\' if in_string => escape = true,
                '"' => in_string = !in_string,
                '{' | '[' if !in_string => depth += 1,
                '}' | ']' if !in_string => {
                    depth -= 1;
                    if depth == 0 {
                        return &s[..=i];
                    }
                }
                _ => {}
            }
        }
        s
    }

    fn parse_json_response(raw: &str) -> String {
        let cleaned = raw
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        // Truncate at the end of the first complete JSON object to strip trailing text
        Self::extract_first_json(cleaned).to_string()
    }

    // ── Step 1: Market Analyzer ─────────────────────────────────────────────

    fn build_market_analyzer_prompt(timeframe: &str) -> String {
        let timeframe_guidance = match timeframe {
            "5min" => "SCALPING: last 3-5 candles matter most. Focus on RSI(9) extremes (<30/>70), volume spikes (>2x avg), engulfing patterns. Ignore noise in older candles.",
            "30min" => "SHORT-TERM SWING: SMA(7) vs SMA(15) crossover is primary trend signal. Confirm with RSI(10) and last 3 candle patterns. Volume must confirm direction.",
            "1h" => "INTRADAY: SMA(10) vs SMA(20) for trend. RSI(14) for momentum. Check if price is extended from SMA (>1% = overextended). Support/resistance from 24h high/low.",
            "6h" => "POSITION: broader trend from SMA(10/20). Key levels from multi-day high/low. Derivatives data weighs heavily — funding rate and open interest shifts signal big moves.",
            "12h" => "MULTI-DAY: macro trend direction is critical. Support = lowest low across data. Resistance = highest high. Funding rate extremes (>0.01 or <-0.01) are strong signals.",
            "24h" => "DAILY: only take trades aligned with the macro trend. News impact is relevant. Derivatives positioning (long_ratio vs short_ratio imbalance >60/40) is a strong signal.",
            _ => "Standard analysis: use SMA crossover for trend, RSI for momentum, volume ratio for confirmation.",
        };

        format!(
            "You are MARKET ANALYZER AI — Stage 1 of a multi-agent trading pipeline.\n\n\
            TASK: Analyze raw market data and produce precise, data-driven JSON.\n\
            You do NOT generate trade signals or recommend actions.\n\
            You must reference ACTUAL NUMBERS from the data in your signals array.\n\n\
            Timeframe: {timeframe}\n\
            {timeframe_guidance}\n\n\
            ANALYSIS RULES FOR EACH SYMBOL:\n\n\
            1. marketBias — determine from these concrete checks:\n\
               - SMA crossover: if sma_fast > sma_slow AND price > sma_fast → bullish\n\
               - SMA crossover: if sma_fast < sma_slow AND price < sma_fast → bearish\n\
               - If price is between SMAs or SMAs are flat → neutral\n\
               - Weight RSI: >60 supports bullish, <40 supports bearish\n\
               - Weight derivatives: long_ratio > 0.55 supports bullish, short_ratio > 0.55 supports bearish\n\
               - If indicators conflict (e.g. bullish SMA + bearish RSI), bias is neutral\n\n\
            2. trendStrength:\n\
               - strong: SMA crossover + RSI confirms + volume confirms + derivatives aligned (3+ factors agree)\n\
               - moderate: 2 factors agree\n\
               - weak: fewer than 2 factors agree, or price is range-bound\n\n\
            3. keyLevels — use ACTUAL numbers from the data:\n\
               - support: the lowest low across the kline data\n\
               - resistance: the highest high across the kline data\n\
               - These MUST be real prices from the data, not estimates\n\n\
            4. momentum:\n\
               - accelerating: RSI > 55 AND rising (last 3 candles closing higher) AND momentum_5_candles positive\n\
               - decelerating: RSI was >60 but falling, or consecutive streak breaking\n\
               - steady: RSI between 40-60 with low volatility\n\
               - exhausted: RSI > 70 or < 30, or exhaustion_signal is not \"none\"\n\n\
            5. volumeProfile:\n\
               - confirming: volume_ratio > 1.2 (recent volume higher than prior)\n\
               - diverging: price moving up but volume_ratio < 0.8, or vice versa\n\
               - spike: last_candle_volume_spike contains \"SPIKE\" or ratio > 2.0\n\n\
            6. derivativesSentiment:\n\
               - bullish: funding_rate < 0 (shorts paying longs) AND orderbook_ratio > 1.1\n\
               - bearish: funding_rate > 0.005 AND orderbook_ratio < 0.9\n\
               - squeeze_risk: funding_rate > 0.01 or < -0.01 with high open interest\n\
               - neutral: otherwise\n\n\
            7. signals: 2-5 SHORT strings (<80 chars each) citing SPECIFIC data:\n\
               - BAD: \"bullish momentum detected\" (too vague)\n\
               - GOOD: \"RSI(14) at 62.3, SMA(10) crossed above SMA(20), volume up 1.4x\"\n\
               - GOOD: \"funding rate -0.008 = short squeeze risk, 63% longs\"\n\
               - Every signal MUST include a number from the data\n\n\
            OUTPUT: Return exactly one valid JSON object. No markdown, no code fences.\n\n\
            {{\"analyses\": [{{\"symbol\": \"BTCUSDT\", \"marketBias\": \"bullish|bearish|neutral\", \
            \"trendStrength\": \"strong|moderate|weak\", \
            \"keyLevels\": {{\"support\": number, \"resistance\": number}}, \
            \"momentum\": \"accelerating|steady|decelerating|exhausted\", \
            \"volumeProfile\": \"confirming|diverging|spike\", \
            \"derivativesSentiment\": \"bullish|bearish|neutral|squeeze_risk\", \
            \"signals\": [\"observation with numbers under 80 chars\", ...]}}]}}",
            timeframe = timeframe,
            timeframe_guidance = timeframe_guidance,
        )
    }

    // ── Step 2: Setup Classifier ─────────────────────────────────────────────

    fn compute_setup_features(
        symbol: &str,
        analysis: &MarketAnalysis,
        snapshot: &MarketSnapshot,
        timeframe: &str,
    ) -> Option<SetupFeatures> {
        let ticker = snapshot.get_ticker(symbol)?;
        let klines = snapshot.get_klines(symbol)?;
        if klines.len() < 5 {
            return None;
        }

        let current_price = ticker.get_last_price();
        let market_bias = analysis.market_bias.as_deref().unwrap_or("neutral");
        let trend = analysis.trend_strength.as_deref().unwrap_or("weak");
        let momentum_val = analysis.momentum.as_deref().unwrap_or("neutral");
        let volume_val = analysis.volume_profile.as_deref().unwrap_or("weak");
        let derivs_val = analysis.derivatives_sentiment.as_deref().unwrap_or("neutral");

        // Determine intended direction: use market_bias first, then infer from
        // trend/momentum/derivatives if bias is neutral
        let intended_direction = match market_bias {
            "bullish" => "LONG",
            "bearish" => "SHORT",
            _ => {
                // Infer direction from secondary signals
                let mut bull_score = 0i32;
                let mut bear_score = 0i32;

                match trend {
                    "strong" | "moderate" => {
                        // Check price action for trend direction
                        let closes: Vec<f64> = klines.iter().map(|k| k.get_close()).collect();
                        let n = closes.len();
                        if n >= 2 && closes[n - 1] > closes[0] {
                            bull_score += 1;
                        } else {
                            bear_score += 1;
                        }
                    }
                    _ => {}
                }
                if momentum_val == "accelerating" { bull_score += 1; }
                if momentum_val == "decelerating" || momentum_val == "exhausted" { bear_score += 1; }
                if derivs_val == "bullish" { bull_score += 1; }
                if derivs_val == "bearish" { bear_score += 1; }

                if bull_score > bear_score { "LONG" }
                else if bear_score > bull_score { "SHORT" }
                else { return None; } // truly no signal
            }
        };

        // Compute support/resistance from klines
        let highs: Vec<f64> = klines.iter().map(|k| k.get_high()).collect();
        let lows: Vec<f64> = klines.iter().map(|k| k.get_low()).collect();
        let closes: Vec<f64> = klines.iter().map(|k| k.get_close()).collect();
        let volumes: Vec<f64> = klines.iter().map(|k| k.get_volume()).collect();
        let n = closes.len();

        let resistance = highs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let support = lows.iter().cloned().fold(f64::INFINITY, f64::min);

        // Compute target/stop based on direction and timeframe
        let target_pct = match timeframe {
            "5min"  => 0.0007,
            "30min" => 0.002,
            "1h"    => 0.005,
            "6h"    => 0.01,
            "12h"   => 0.018,
            "24h"   => 0.03,
            _       => 0.005,
        };
        let stop_pct = target_pct * 0.5;

        let (entry_price, target_price, stop_loss) = if intended_direction == "LONG" {
            (
                current_price,
                (current_price * (1.0 + target_pct)).min(resistance),
                current_price * (1.0 - stop_pct),
            )
        } else {
            (
                current_price,
                (current_price * (1.0 - target_pct)).max(support),
                current_price * (1.0 + stop_pct),
            )
        };

        let reward = (target_price - entry_price).abs();
        let risk = (stop_loss - entry_price).abs();
        let risk_reward = if risk > 0.0 { reward / risk } else { 0.0 };

        let dist_to_resistance = if resistance > 0.0 {
            ((resistance - current_price) / current_price) * 100.0
        } else {
            0.0
        };
        let dist_to_support = if support > 0.0 {
            ((current_price - support) / current_price) * 100.0
        } else {
            0.0
        };

        // Volatility spike: last candle volume vs avg (use 3x threshold, 2x was too sensitive)
        let vol_spike = if n >= 2 {
            let last_vol = volumes[n - 1];
            let avg_vol = if n >= 6 {
                volumes[n - 6..n - 1].iter().sum::<f64>() / 5.0
            } else {
                volumes[..n - 1].iter().sum::<f64>() / (n - 1) as f64
            };
            if avg_vol > 0.0 { last_vol / avg_vol > 3.0 } else { false }
        } else {
            false
        };

        // Leverage/liquidation risk from derivatives
        let derivs = snapshot.get_derivatives(symbol);
        let funding_rate = derivs.map(|d| d.get_funding_rate()).unwrap_or(0.0);
        let long_ratio = derivs.map(|d| d.get_long_ratio()).unwrap_or(0.5);
        let short_ratio = derivs.map(|d| d.get_short_ratio()).unwrap_or(0.5);

        let leverage_risk = funding_rate.abs() > 0.01;
        let liquidation_risk = (intended_direction == "LONG" && long_ratio > 0.70 && funding_rate > 0.005)
            || (intended_direction == "SHORT" && short_ratio > 0.70 && funding_rate < -0.005);

        // RSI
        let rsi_period = match timeframe {
            "5min" => 9,
            "30min" => 10,
            _ => 14,
        };
        let rsi = compute_rsi_for_features(&closes, rsi_period);

        // SMA crossover
        let sma_fast_period = if n >= 10 { 10 } else { n };
        let sma_slow_period = if n >= 20 { 20 } else { n };
        let sma_fast = closes[n - sma_fast_period..].iter().sum::<f64>() / sma_fast_period as f64;
        let sma_slow = closes[n - sma_slow_period..].iter().sum::<f64>() / sma_slow_period as f64;

        // Count confirmations (broader criteria)
        let mut confirmations = Vec::new();

        // 1. Trend alignment
        if trend == "strong" || trend == "moderate" {
            confirmations.push("trend_aligned".to_string());
        }
        // 2. Market bias matches direction
        if (intended_direction == "LONG" && market_bias == "bullish")
            || (intended_direction == "SHORT" && market_bias == "bearish")
        {
            confirmations.push("bias_aligned".to_string());
        }
        // 3. Momentum (broader: accelerating=bullish, decelerating/exhausted=bearish)
        if (intended_direction == "LONG" && (momentum_val == "accelerating" || momentum_val == "steady"))
            || (intended_direction == "SHORT" && (momentum_val == "decelerating" || momentum_val == "exhausted"))
        {
            confirmations.push("momentum_confirming".to_string());
        }
        // 4. Volume
        if volume_val == "confirming" {
            confirmations.push("volume_confirming".to_string());
        }
        // 5. Derivatives sentiment
        if (intended_direction == "LONG" && derivs_val == "bullish")
            || (intended_direction == "SHORT" && derivs_val == "bearish")
        {
            confirmations.push("derivatives_aligned".to_string());
        }
        // 6. RSI favorable (wider range: < 45 for long, > 55 for short)
        if (intended_direction == "LONG" && rsi < 45.0)
            || (intended_direction == "SHORT" && rsi > 55.0)
        {
            confirmations.push("rsi_favorable".to_string());
        }
        // 7. SMA crossover
        if (intended_direction == "LONG" && sma_fast > sma_slow)
            || (intended_direction == "SHORT" && sma_fast < sma_slow)
        {
            confirmations.push("sma_crossover".to_string());
        }
        // 8. Price position relative to SMAs
        if (intended_direction == "LONG" && current_price > sma_fast)
            || (intended_direction == "SHORT" && current_price < sma_fast)
        {
            confirmations.push("price_above_sma".to_string());
        }

        Some(SetupFeatures {
            symbol: symbol.to_string(),
            intended_direction: intended_direction.to_string(),
            entry_price,
            target_price,
            stop_loss,
            risk_reward,
            distance_to_resistance_pct: dist_to_resistance,
            distance_to_support_pct: dist_to_support,
            volatility_spike: vol_spike,
            leverage_risk,
            liquidation_risk,
            confirmations_count: confirmations.len() as u32,
            confirmations,
            market_bias: market_bias.to_string(),
            trend_strength: trend.to_string(),
            momentum: momentum_val.to_string(),
            volume_profile: volume_val.to_string(),
            rsi,
        })
    }

    fn build_setup_classifier_prompt(features_json: &str, timeframe: &str, count: usize) -> String {
        let time_horizon = match timeframe {
            "5min"  => "5-15 MINUTES",
            "30min" => "30-90 MINUTES",
            "1h"    => "1-3 HOURS",
            "6h"    => "6-18 HOURS",
            "12h"   => "12-36 HOURS",
            "24h"   => "1-3 DAYS",
            _       => "1-3 candles",
        };

        let output_instruction = if count == 1 {
            "Return exactly one JSON object for the single setup provided.".to_string()
        } else {
            format!(
                "You receive {} setups. Return a JSON object with a \"signals\" array \
                containing one classification per setup, in the same order. \
                Every setup MUST have a corresponding entry in the array.",
                count
            )
        };

        let output_schema = if count == 1 {
            "{{\"symbol\":\"...\",\"decision\":\"LONG|SHORT|NO_TRADE\",\
            \"status\":\"APPROVED|REDUCED_SIZE|ACCEPT_WITH_CAUTION|WAIT_CONFIRMATION|REJECTED\",\
            \"confidence\":0-100,\"riskReward\":number,\
            \"confluenceScore\":0.0-1.0,\
            \"reasoning\":[\"factor 1\",\"factor 2\",...],\
            \"issues\":[\"risk factor\",...]}}".to_string()
        } else {
            "{{\"signals\":[{{\"symbol\":\"...\",\"decision\":\"LONG|SHORT|NO_TRADE\",\
            \"status\":\"APPROVED|REDUCED_SIZE|ACCEPT_WITH_CAUTION|WAIT_CONFIRMATION|REJECTED\",\
            \"confidence\":0-100,\"riskReward\":number,\
            \"confluenceScore\":0.0-1.0,\
            \"reasoning\":[\"factor 1\",...],\
            \"issues\":[\"risk factor\",...]}}, ...]}}".to_string()
        };

        format!(
            "You are Setup Classifier AI.\n\
            You receive PRE-COMPUTED deterministic features for potential trade setups.\n\
            Your job: classify each setup's quality using ONLY the numbers provided.\n\
            Do NOT invent data. Do NOT analyze raw market data. Use the features as-is.\n\n\
            === COMPUTED FEATURES ===\n{features_json}\n\n\
            TIMEFRAME: {timeframe} | HORIZON: {time_horizon}\n\n\
            ---\n\
            STEP 1: CHECK HARD REJECTION RULES (in order, stop at first trigger):\n\
            1. riskReward < 1.2 → NO_TRADE (cite the exact riskReward value)\n\
            2. confirmationsCount < 2 → NO_TRADE (cite the count and list which confirmations exist)\n\
            3. volatilitySpike == true AND leverageRisk == true → NO_TRADE\n\
            4. liquidationRisk == true → NO_TRADE\n\n\
            ---\n\
            STEP 2: CLASSIFY (only if no hard rule triggered):\n\n\
            APPROVED (confidence 70-90):\n\
            - riskReward >= 1.8 AND confirmationsCount >= 4\n\
            - Reasoning must list each confirmation by name\n\n\
            REDUCED_SIZE (confidence 55-69):\n\
            - riskReward >= 1.5 AND confirmationsCount >= 3\n\
            - OR any risk flag is true (leverageRisk)\n\n\
            ACCEPT_WITH_CAUTION (confidence 40-54):\n\
            - confirmationsCount == 2 or 3 with riskReward between 1.2 and 1.5\n\
            - OR volatilitySpike == true\n\n\
            WAIT_CONFIRMATION (confidence 30-39):\n\
            - Direction looks valid but momentum is \"neutral\" or \"steady\"\n\
            - confirmationsCount == 2 with weak volume\n\n\
            ---\n\
            CONFIDENCE FORMULA (follow exactly):\n\
            base = confirmationsCount * 12\n\
            if riskReward >= 2.0: base += 10\n\
            if leverageRisk: base -= 10\n\
            if volatilitySpike: base -= 15\n\
            confidence = clamp(base, 10, 90)\n\n\
            ---\n\
            confluenceScore = confirmationsCount / 8.0 (capped at 1.0). Compute exactly.\n\n\
            ---\n\
            REASONING RULES:\n\
            - Each reasoning item must cite a specific feature name and its value\n\
            - BAD: \"good risk reward\" → GOOD: \"riskReward 2.1 exceeds 1.8 threshold\"\n\
            - BAD: \"trend is aligned\" → GOOD: \"trend_aligned + sma_crossover + volume_confirming = 3 confirmations\"\n\
            - issues array: list each risk factor with its value\n\n\
            ---\n\
            OUTPUT FORMAT:\n\
            {output_instruction}\n\
            No markdown, no code fences. All strings closed, all braces balanced.\n\n\
            OUTPUT SCHEMA:\n\
            {output_schema}",
            features_json = features_json,
            timeframe = timeframe,
            time_horizon = time_horizon,
        )
    }

    // ── Step 3: Risk Manager ────────────────────────────────────────────────

    fn build_risk_manager_prompt(market_json: &str, signal_json: &str, timeframe: &str, bet_value: f64) -> String {
        format!(
            "You are RISK MANAGER AI — Stage 3 of a multi-agent trading pipeline.\n\n\
            You receive Market Analysis (Stage 1) and Setup Classification (Stage 2).\n\
            Your job: validate the trade's risk profile using concrete checks.\n\n\
            === MARKET ANALYSIS (Stage 1) ===\n{market_json}\n\n\
            === SIGNALS (Stage 2) ===\n{signal_json}\n\n\
            TIMEFRAME: {timeframe}\n\
            BET VALUE: ${bet_value:.2}\n\n\
            CONCRETE RISK CHECKS (evaluate each, cite numbers):\n\n\
            1. STOP LOSS VALIDATION:\n\
               - For LONG: stop must be BELOW entry price\n\
               - For SHORT: stop must be ABOVE entry price\n\
               - Stop distance must be >= 0.05% of entry (not too tight)\n\
               - Stop should be near a support/resistance level from Stage 1 keyLevels\n\
               - If stop is in empty space (not near a level), note this as a risk\n\n\
            2. TARGET VALIDATION:\n\
               - For LONG: target must be ABOVE entry. For SHORT: target must be BELOW entry\n\
               - Target should not exceed the next major resistance (LONG) or support (SHORT)\n\
               - Compute actual R:R = |target - entry| / |stop - entry|. Report this number.\n\n\
            3. DERIVATIVES RISK:\n\
               - funding_rate > 0.005 on a LONG = crowded trade risk → REDUCE_SIZE\n\
               - funding_rate < -0.005 on a SHORT = crowded trade risk → REDUCE_SIZE\n\
               - long_ratio > 0.65 on a LONG = overcrowded → REDUCE_SIZE\n\
               - squeeze_risk detected = high risk → REDUCE_SIZE or REJECT\n\n\
            4. VOLUME RISK:\n\
               - If volumeProfile is \"diverging\" or \"spike\" → REDUCE_SIZE\n\
               - Low volume (volume_ratio < 0.7) means weak conviction → REDUCE_SIZE\n\n\
            5. POSITION SIZING:\n\
               - APPROVE + no flags: positionSizePct = 80-100\n\
               - REDUCE_SIZE (1 risk flag): positionSizePct = 50-75\n\
               - REDUCE_SIZE (2+ risk flags): positionSizePct = 25-50\n\
               - REJECT: positionSizePct = 0\n\n\
            DECISIONS:\n\
            - APPROVE: all checks pass, risk is acceptable\n\
            - REDUCE_SIZE: direction is valid but 1+ risk flags detected\n\
            - REJECT: stop/target are invalid, R:R < 1.2, or 3+ risk flags\n\n\
            riskNotes MUST cite specific numbers (e.g. \"R:R 1.3 below minimum 1.5\", \"funding 0.008 = crowded long\").\n\n\
            OUTPUT: one valid JSON object, no markdown.\n\n\
            {{\"assessments\": [{{\"symbol\": \"BTCUSDT\", \"decision\": \"APPROVE|REDUCE_SIZE|REJECT\", \
            \"riskRewardRatio\": number, \"positionSizePct\": 0-100, \
            \"riskNotes\": \"specific risk assessment with numbers\"}}]}}",
            market_json = market_json,
            signal_json = signal_json,
            timeframe = timeframe,
            bet_value = bet_value,
        )
    }

    // ── Step 4: Strategy Optimizer ───────────────────────────────────────────

    fn build_strategy_optimizer_prompt(
        market_json: &str,
        signal_json: &str,
        risk_json: &str,
        timeframe: &str,
        bet_value: f64,
    ) -> String {
        format!(
            "You are STRATEGY OPTIMIZER AI — Stage 4 of a multi-agent trading pipeline.\n\n\
            You receive all previous stage outputs. Produce the final optimized execution plan.\n\n\
            === MARKET ANALYSIS (Stage 1) ===\n{market_json}\n\n\
            === SIGNALS (Stage 2) ===\n{signal_json}\n\n\
            === RISK ASSESSMENT (Stage 3) ===\n{risk_json}\n\n\
            TIMEFRAME: {timeframe}\n\
            BET VALUE: ${bet_value:.2}\n\n\
            OPTIMIZATION RULES:\n\n\
            1. ENTRY ADJUSTMENT:\n\
               - If current price is far from support (LONG) or resistance (SHORT), \
            consider waiting for a pullback → WAIT_CONFIRMATION\n\
               - If price is AT a key level → ENTER_NOW is appropriate\n\
               - adjustedEntry should be the CURRENT price from Stage 1 data unless you have a reason to change it\n\
               - Never adjust entry more than 0.5% from the signal's entry price\n\n\
            2. TARGET ADJUSTMENT:\n\
               - Target should not exceed keyLevels.resistance (LONG) or keyLevels.support (SHORT)\n\
               - If signal target exceeds the next key level, pull it back to that level\n\
               - Do not make targets more aggressive than the signal\n\n\
            3. STOP ADJUSTMENT:\n\
               - Move stop to just beyond the nearest support (LONG) or resistance (SHORT)\n\
               - Never widen stop more than 0.3% beyond the signal's stop\n\
               - Tighter stop is acceptable if there's a clear level nearby\n\n\
            4. EXECUTION ACTION LOGIC:\n\
               - ENTER_NOW: momentum is \"accelerating\", volume \"confirming\", no risk flags\n\
               - WAIT_CONFIRMATION: momentum \"steady\" or \"neutral\", or volume not confirming\n\
               - SCALE_IN: trade looks good but some uncertainty — enter 40% now, 60% on confirmation\n\
               - REDUCED_SIZE: risk manager said REDUCE_SIZE, or 1+ issues flagged\n\
               - SKIP_TRADE: risk manager REJECTED, or you detect a fatal flaw\n\n\
            5. POSITION SIZING:\n\
               - Respect the risk manager's positionSizePct as a ceiling\n\
               - ENTER_NOW: use risk manager's size\n\
               - SCALE_IN: use 40-50% of risk manager's size for first entry\n\
               - REDUCED_SIZE: use 50-70% of risk manager's size\n\n\
            executionNotes MUST explain WHY this action (cite data from previous stages).\n\n\
            OUTPUT: one valid JSON object, no markdown.\n\n\
            {{\"strategies\": [{{\"symbol\": \"BTCUSDT\", \
            \"executionAction\": \"ENTER_NOW|WAIT_CONFIRMATION|SCALE_IN|REDUCED_SIZE|SKIP_TRADE\", \
            \"adjustedEntry\": number, \"adjustedTarget\": number, \"adjustedStop\": number, \
            \"adjustedPositionSizePct\": 0-100, \
            \"executionNotes\": \"specific reasoning with data references\"}}]}}",
            market_json = market_json,
            signal_json = signal_json,
            risk_json = risk_json,
            timeframe = timeframe,
            bet_value = bet_value,
        )
    }

    // ── Step 5: Review AI (Haiku) ───────────────────────────────────────────

    fn build_review_prompt(
        market_json: &str,
        signal_json: &str,
        risk_json: &str,
        strategy_json: &str,
    ) -> String {
        format!(
            "STEP 5 — REVIEW AI\n\n\
            ROLE\n\
            You are the final adjudication and validation layer in a multi-step crypto AI pipeline.\n\n\
            PURPOSE\n\
            Your job is to review the outputs of the previous agents, detect weak reasoning or contradictions, \
            and produce a final usable decision.\n\
            You are NOT a passive summarizer.\n\
            You are NOT allowed to return an unreasoned rejection.\n\
            You must always return a fully populated structured result.\n\n\
            INPUTS\n\
            === MARKET ANALYSIS (Stage 1) ===\n{market_json}\n\n\
            === SIGNALS (Stage 2) ===\n{signal_json}\n\n\
            === RISK ASSESSMENT (Stage 3) ===\n{risk_json}\n\n\
            === STRATEGY (Stage 4) ===\n{strategy_json}\n\n\
            PRIMARY RESPONSIBILITIES\n\
            1. Check logical consistency — verify that previous agent outputs are internally consistent.\n\
            2. Detect contradictions — find conflicts between market bias, execution plan, risk stance, \
            target/invalidation quality, confidence levels.\n\
            3. Detect weak reasoning — identify unsupported, shallow, generic, or overconfident conclusions.\n\
            4. Validate execution quality — determine whether the final execution plan is justified.\n\
            5. Final adjudication — you must always return one final actionable result: LONG, SHORT, or NO_TRADE.\n\
            6. Confidence adjustment — if evidence is mixed, reduce confidence instead of collapsing into empty rejection.\n\n\
            IMPORTANT DECISION RULES\n\
            - Do not default to REJECT. Do not use REJECT as a fallback.\n\
            - Minor disagreement between earlier agents is NOT enough for REJECT.\n\
            - A weak entry is not the same as a weak market thesis.\n\
            - Directional bias and execution timing are separate outputs.\n\
            - If directional bias is still valid but entry quality is weak, prefer: \
            ACCEPT_WITH_CAUTION, DOWNGRADE, WAIT_CONFIRMATION, or REDUCED_SIZE.\n\
            - Use REJECT only when the setup is structurally invalid.\n\
            - Use NO_TRADE only when: contradictions are severe, confidence is below threshold, \
            execution quality is unacceptable, or targets/invalidation are structurally broken.\n\n\
            STRICT OUTPUT RULES\n\
            1. Always return a fully populated JSON object.\n\
            2. Never return empty arrays for both detectedIssues and reviewNotes.\n\
            3. If finalVerdict is ACCEPT: reviewNotes must contain at least 1 item.\n\
            4. If finalVerdict is ACCEPT_WITH_CAUTION or DOWNGRADE: detectedIssues must contain at least 1 concrete issue, \
            reviewNotes must contain at least 1 concrete note.\n\
            5. If finalVerdict is REJECT: finalDecision must be NO_TRADE, detectedIssues must contain at least 2 concrete issues, \
            reviewNotes must contain at least 1 explanatory note, finalApprovedPlan must still be fully populated.\n\
            6. Never use vague phrases without specifics (e.g. 'mixed signals', 'uncertain market') unless you explain the exact cause.\n\
            7. Every detected issue must reference which prior agent output caused it.\n\n\
            VERDICT DEFINITIONS\n\
            - ACCEPT: thesis and execution are acceptable.\n\
            - ACCEPT_WITH_CAUTION: thesis is valid, but some moderate risk exists.\n\
            - DOWNGRADE: thesis may still be valid, but confidence, sizing, or execution quality must be reduced.\n\
            - REJECT: thesis or execution is structurally invalid, so finalDecision must be NO_TRADE.\n\n\
            REVIEW THRESHOLD LOGIC — prefer this downgrade ladder:\n\
            1. ACCEPT → 2. ACCEPT_WITH_CAUTION → 3. DOWNGRADE → 4. REJECT\n\
            Use the least severe valid outcome. Do not jump to REJECT unless clearly justified.\n\n\
            DIRECTIONAL THESIS RULE\n\
            Do not confuse market direction with execution timing.\n\
            - marketBias can remain LONG while executionPlan is WAIT_CONFIRMATION.\n\
            - marketBias can remain SHORT while riskDecision is REDUCE_SIZE.\n\
            - Only set finalDecision to NO_TRADE if the setup should truly not be taken.\n\n\
            For each symbol, output your review.\n\n\
            OUTPUT RULES:\n\
            - Return exactly one valid JSON object\n\
            - No markdown, no code fences, no commentary\n\
            - All strings must be closed, all braces balanced\n\
            - Keep text fields under 100 characters\n\
            - Use arrays of short strings, not long paragraphs\n\n\
            {{\"reviews\": [{{\"reviewResult\": {{\"consistencyStatus\": \"PASS|WARNING|FAIL\", \
            \"finalVerdict\": \"ACCEPT|ACCEPT_WITH_CAUTION|DOWNGRADE|REJECT\", \
            \"finalDecision\": \"LONG|SHORT|NO_TRADE\", \"confidence\": 0-100}}, \
            \"detectedIssues\": [{{\"source\": \"MarketAnalyzer|SignalGenerator|RiskManager|StrategyOptimizer\", \
            \"issue\": \"description\"}}], \
            \"reviewNotes\": [\"note1\", ...], \
            \"finalApprovedPlan\": {{\"marketBias\": \"LONG|SHORT|NO_TRADE\", \
            \"executionPlan\": \"ENTER_NOW|WAIT_CONFIRMATION|SCALE_IN|REDUCED_SIZE|SKIP_TRADE\", \
            \"setupType\": \"breakout|mean_reversion|trend_continuation|squeeze|no_trade\", \
            \"targets\": {{\"primary\": number, \"secondary\": number}}, \
            \"invalidation\": number, \"riskDecision\": \"APPROVE|REDUCE_SIZE|REJECT\"}}}}]}}",
            market_json = market_json,
            signal_json = signal_json,
            risk_json = risk_json,
            strategy_json = strategy_json,
        )
    }

    // ── Main pipeline ───────────────────────────────────────────────────────

    pub async fn analyze(
        &self,
        snapshot: &MarketSnapshot,
        timeframe: &str,
        bet_value: f64,
    ) -> Result<Vec<Prediction>, Box<dyn std::error::Error + Send + Sync>> {
        if self.api_key.is_empty() {
            return Err("AI_API_KEY not set. Set your Anthropic API key.".into());
        }

        let tickers_json = snapshot.tickers_to_json();
        let klines_json = snapshot.klines_to_json();
        let news_json = snapshot.news_to_json();
        let indicators_json = snapshot.compute_indicators(timeframe);
        let derivatives_json = snapshot.derivatives_to_json();

        let user_content = AnalysisService::build_analysis_prompt(
            &tickers_json, &klines_json, &news_json, &indicators_json, &derivatives_json, timeframe,
        );

        // ── STEP 1: Market Analyzer ─────────────────────────────────────────
        let step1_system = Self::build_market_analyzer_prompt(timeframe);
        tracing::info!("Pipeline Step 1: Market Analyzer ({})", self.fast_model);
        let step1_raw = self
            .call_model_with_retry(&self.fast_model, &step1_system, &user_content, 4096)
            .await?;
        let step1_json = Self::parse_json_response(&step1_raw);
        tracing::info!("Step 1 complete: {} chars", step1_json.len());

        let analyses: Vec<MarketAnalysis> = match serde_json::from_str::<AnalysisResponse>(&step1_json) {
            Ok(r) => r.analyses.unwrap_or_default(),
            Err(e) => {
                tracing::warn!("Step 1 parse failed ({}), retrying...", e);
                let retry_raw = self
                    .call_model_with_retry(&self.fast_model, &step1_system, &user_content, 4096)
                    .await?;
                let retry_json = Self::parse_json_response(&retry_raw);
                match serde_json::from_str::<AnalysisResponse>(&retry_json) {
                    Ok(r) => r.analyses.unwrap_or_default(),
                    Err(e2) => {
                        tracing::error!("Step 1 retry parse failed: {}. Returning fallback.", e2);
                        let fallback_symbol = snapshot.first_symbol().unwrap_or_else(|| "UNKNOWN".into());
                        let fallback = Prediction::new(
                            &fallback_symbol, "NO_TRADE", 0.0,
                            "Market analysis unavailable", 0.0, 0.0, 0.0,
                            None, None, None, None, Some(timeframe.to_string()),
                        ).with_pipeline(
                            None, None, None, None, None, None, None, None,
                            None, None, None, None, None, None, None, None, None, None,
                            Some("REJECTED".into()),
                            None,
                            Some("Market analysis could not be parsed".into()),
                            None, None,
                        );
                        return Ok(vec![fallback]);
                    }
                }
            }
        };

        if analyses.is_empty() {
            tracing::error!("Step 1 returned empty analyses. Returning fallback.");
            let fallback_symbol = snapshot.first_symbol().unwrap_or_else(|| "UNKNOWN".into());
            let fallback = Prediction::new(
                &fallback_symbol, "NO_TRADE", 0.0,
                "Market analysis unavailable", 0.0, 0.0, 0.0,
                None, None, None, None, Some(timeframe.to_string()),
            ).with_pipeline(
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None,
                Some("REJECTED".into()),
                None,
                Some("Market analyzer returned no analyses".into()),
                None, None,
            );
            return Ok(vec![fallback]);
        }

        // 1-2s delay between AI calls
        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        // ── STEP 2: Setup Classifier ──────────────────────────────────────
        // Compute features for ALL analyzed symbols
        let mut all_features: Vec<SetupFeatures> = Vec::new();

        for analysis in &analyses {
            match Self::compute_setup_features(&analysis.symbol, analysis, snapshot, timeframe) {
                Some(f) => {
                    tracing::info!("{}: {} confirmations, R:R {:.2}, dir {}", f.symbol, f.confirmations_count, f.risk_reward, f.intended_direction);
                    all_features.push(f);
                }
                None => {
                    tracing::info!("{}: no directional signal — skipping", analysis.symbol);
                }
            }
        }

        // If no symbol produced features → return NO_TRADE
        if all_features.is_empty() {
            tracing::info!("No symbols have directional bias — returning NO_TRADE");
            let first_analysis = analyses.first();
            let fallback_symbol = first_analysis
                .map(|a| a.symbol.clone())
                .or_else(|| snapshot.first_symbol())
                .unwrap_or_else(|| "UNKNOWN".into());
            let market_bias = first_analysis.and_then(|a| a.market_bias.clone());
            let trend_strength = first_analysis.and_then(|a| a.trend_strength.clone());
            let momentum = first_analysis.and_then(|a| a.momentum.clone());
            let volume_profile = first_analysis.and_then(|a| a.volume_profile.clone());
            let derivatives_sentiment = first_analysis.and_then(|a| a.derivatives_sentiment.clone());
            let market_signals = first_analysis.and_then(|a| a.signals.clone());

            let fallback = Prediction::new(
                &fallback_symbol, "NO_TRADE", 0.0,
                "No directional bias detected across all pairs", 0.0, 0.0, 0.0,
                None, None, None, None, Some(timeframe.to_string()),
            ).with_pipeline(
                market_bias, None, None, None, None, None, None, None,
                None, None, None, None, None, None,
                trend_strength, momentum, volume_profile, derivatives_sentiment,
                Some("REJECTED".into()),
                market_signals,
                Some("No directional bias across all pairs".into()),
                None, None,
            );
            return Ok(vec![fallback]);
        }

        let feature_count = all_features.len();
        tracing::info!("Classifying {} symbols", feature_count);

        // Build features JSON: single object for 1, array for many
        let features_json = if feature_count == 1 {
            serde_json::to_string_pretty(&all_features[0]).unwrap_or_else(|_| "{}".into())
        } else {
            serde_json::to_string_pretty(&all_features).unwrap_or_else(|_| "[]".into())
        };

        let step2_system = Self::build_setup_classifier_prompt(&features_json, timeframe, feature_count);
        tracing::info!("Pipeline Step 2: Setup Classifier ({}) for {} symbols", self.model, feature_count);
        let step2_raw = self
            .call_model_with_retry(&self.model, &step2_system, &user_content, 4096)
            .await?;

        // Parse response: single object or {"signals":[...]} depending on count
        let mut signals: Vec<SignalOutput> = if feature_count == 1 {
            let step2_cleaned = Self::parse_json_response(&step2_raw);
            match serde_json::from_str::<SignalOutput>(&step2_cleaned) {
                Ok(signal) => vec![signal],
                Err(e) => {
                    tracing::warn!("Step 2 single parse failed ({}), retrying...", e);
                    let retry_raw = self
                        .call_model_with_retry(&self.model, &step2_system, &user_content, 4096)
                        .await?;
                    let retry_cleaned = Self::parse_json_response(&retry_raw);
                    serde_json::from_str::<SignalOutput>(&retry_cleaned)
                        .map(|s| vec![s])
                        .unwrap_or_default()
                }
            }
        } else {
            let step2_json_str = Self::parse_json_response(&step2_raw);
            match serde_json::from_str::<ClassifierResponse>(&step2_json_str) {
                Ok(r) => r.signals.unwrap_or_default(),
                Err(e) => {
                    tracing::warn!("Step 2 array parse failed ({}), retrying...", e);
                    let retry_raw = self
                        .call_model_with_retry(&self.model, &step2_system, &user_content, 4096)
                        .await?;
                    let retry_json = Self::parse_json_response(&retry_raw);
                    serde_json::from_str::<ClassifierResponse>(&retry_json)
                        .map(|r| r.signals.unwrap_or_default())
                        .unwrap_or_default()
                }
            }
        };

        if signals.is_empty() {
            tracing::error!("Step 2 produced no signals. Returning fallback.");
            let sym = all_features.first().map(|f| f.symbol.as_str()).unwrap_or("UNKNOWN");
            let fallback = Prediction::new(
                sym, "NO_TRADE", 0.0,
                "Setup classifier returned empty response", 0.0, 0.0, 0.0,
                None, None, None, None, Some(timeframe.to_string()),
            ).with_pipeline(
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None,
                Some("REJECTED".into()),
                None,
                Some("Setup classifier returned empty response".into()),
                None, None,
            );
            return Ok(vec![fallback]);
        }

        // Enrich classifier output with computed levels from features
        for signal in &mut signals {
            if let Some(feat) = all_features.iter().find(|f| f.symbol == signal.symbol) {
                if signal.entry_price.is_none() {
                    signal.entry_price = Some(feat.entry_price);
                }
                if signal.target_price.is_none() {
                    signal.target_price = Some(feat.target_price);
                }
                if signal.stop_loss.is_none() {
                    signal.stop_loss = Some(feat.stop_loss);
                }
                if signal.risk_reward.is_none() {
                    signal.risk_reward = Some(feat.risk_reward);
                }
            }
        }

        // Re-serialize signals as {"signals":[...]} for downstream stages 3-5
        let step2_json = serde_json::to_string(&serde_json::json!({"signals": signals}))
            .unwrap_or_else(|_| "{}".into());
        tracing::info!("Step 2 complete: {} signals, {} chars", signals.len(), step2_json.len());

        // Separate tradeable signals from NO_TRADE
        let mut no_trade_predictions: Vec<Prediction> = Vec::new();
        let tradeable_signals: Vec<SignalOutput> = signals.into_iter().filter(|s| {
            if s.decision.as_deref() == Some("NO_TRADE") {
                let analysis = analyses.iter().find(|a| a.symbol == s.symbol).or(analyses.first());
                let market_bias = analysis.and_then(|a| a.market_bias.clone());
                let trend_strength = analysis.and_then(|a| a.trend_strength.clone());
                let momentum = analysis.and_then(|a| a.momentum.clone());
                let volume_profile = analysis.and_then(|a| a.volume_profile.clone());
                let derivatives_sentiment = analysis.and_then(|a| a.derivatives_sentiment.clone());
                let market_signals = analysis.and_then(|a| a.signals.clone());
                let reasoning = s.reasoning.as_ref()
                    .map(|v| v.join("\n"))
                    .unwrap_or_else(|| "No valid trade setup found".into());

                let pred = Prediction::new(
                    &s.symbol, "NO_TRADE", s.confidence.unwrap_or(0.0),
                    &reasoning, 0.0, 0.0, 0.0,
                    None, None, None, None, Some(timeframe.to_string()),
                ).with_pipeline(
                    market_bias, None, None, s.risk_reward, None,
                    None, None, None, None, None, None, None, None, None,
                    trend_strength, momentum, volume_profile, derivatives_sentiment,
                    Some("REJECTED".into()),
                    market_signals,
                    Some("No valid trade setup identified".into()),
                    s.confluence_score,
                    s.issues.clone(),
                );
                no_trade_predictions.push(pred);
                false
            } else {
                true
            }
        }).collect();

        // If all signals are NO_TRADE, return the NO_TRADE predictions
        if tradeable_signals.is_empty() {
            tracing::info!("All {} signals are NO_TRADE — returning REJECTED predictions", no_trade_predictions.len());
            return Ok(no_trade_predictions);
        }

        let signals = tradeable_signals;

        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        // ── STEP 3: Risk Manager ────────────────────────────────────────────
        let step3_system = Self::build_risk_manager_prompt(&step1_json, &step2_json, timeframe, bet_value);
        tracing::info!("Pipeline Step 3: Risk Manager ({})", self.fast_model);
        let step3_raw = self
            .call_model(&self.fast_model, &step3_system, &user_content, 3000)
            .await?;
        let step3_json = Self::parse_json_response(&step3_raw);
        tracing::info!("Step 3 complete: {} chars", step3_json.len());

        let risks: Vec<RiskAssessment> = serde_json::from_str::<RiskResponse>(&step3_json)
            .map(|r| r.assessments.unwrap_or_default())
            .unwrap_or_else(|e| {
                tracing::warn!("Step 3 parse warning: {}", e);
                vec![]
            });

        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        // ── STEP 4: Strategy Optimizer ──────────────────────────────────────
        let step4_system = Self::build_strategy_optimizer_prompt(&step1_json, &step2_json, &step3_json, timeframe, bet_value);
        tracing::info!("Pipeline Step 4: Strategy Optimizer ({})", self.fast_model);
        let step4_raw = self
            .call_model(&self.fast_model, &step4_system, &user_content, 3000)
            .await?;
        let step4_json = Self::parse_json_response(&step4_raw);
        tracing::info!("Step 4 complete: {} chars", step4_json.len());

        let strategies: Vec<StrategyOutput> = serde_json::from_str::<StrategyResponse>(&step4_json)
            .map(|r| r.strategies.unwrap_or_default())
            .unwrap_or_else(|e| {
                tracing::warn!("Step 4 parse warning: {}", e);
                vec![]
            });

        tokio::time::sleep(std::time::Duration::from_millis(1500)).await;

        // ── STEP 5: Review AI (Haiku) ───────────────────────────────────────
        let step5_system = Self::build_review_prompt(&step1_json, &step2_json, &step3_json, &step4_json);
        tracing::info!("Pipeline Step 5: Review AI ({})", self.review_model);
        let step5_result = self
            .call_model(&self.review_model, &step5_system, &user_content, 3000)
            .await;

        let reviews: Vec<ReviewResult> = match step5_result {
            Ok(raw) => {
                let json = Self::parse_json_response(&raw);
                tracing::info!("Step 5 complete: {} chars", json.len());
                serde_json::from_str::<ReviewResponse>(&json)
                    .map(|r| r.reviews.unwrap_or_default())
                    .unwrap_or_else(|e| {
                        tracing::warn!("Step 5 parse warning: {}", e);
                        vec![]
                    })
            }
            Err(e) => {
                tracing::warn!("Review AI failed (proceeding without): {}", e);
                vec![]
            }
        };

        // ── Merge all stages into Predictions ───────────────────────────────
        let predictions = self.merge_pipeline(
            &analyses, &signals, &risks, &strategies, &reviews, timeframe,
        );

        tracing::info!("Pipeline complete: {} final predictions", predictions.len());
        Ok(predictions)
    }

    fn merge_pipeline(
        &self,
        analyses: &[MarketAnalysis],
        signals: &[SignalOutput],
        risks: &[RiskAssessment],
        strategies: &[StrategyOutput],
        reviews: &[ReviewResult],
        timeframe: &str,
    ) -> Vec<Prediction> {
        signals
            .iter()
            .filter_map(|signal| {
                let symbol = &signal.symbol;
                let direction = signal.decision.as_deref()?;

                // Skip NO_TRADE
                if direction == "NO_TRADE" {
                    return None;
                }

                let confidence = signal.confidence.unwrap_or(50.0);
                let entry_price = signal.entry_price?;
                let mut target_price = signal.target_price?;
                let mut stop_loss = signal.stop_loss?;
                let secondary_target: Option<f64> = None;
                let invalidation: Option<f64> = None;
                let setup_type: Option<String> = None;
                let reasoning = signal.reasoning.as_ref()
                    .map(|v| v.join("\n"))
                    .unwrap_or_else(|| "No reasoning".into());

                // Get market analysis
                let analysis = analyses.iter().find(|a| a.symbol == *symbol);
                let market_bias = analysis.and_then(|a| a.market_bias.clone());
                let trend_strength = analysis.and_then(|a| a.trend_strength.clone());
                let momentum = analysis.and_then(|a| a.momentum.clone());
                let volume_profile = analysis.and_then(|a| a.volume_profile.clone());
                let derivatives_sentiment = analysis.and_then(|a| a.derivatives_sentiment.clone());
                let market_signals = analysis.and_then(|a| a.signals.clone());

                // Get risk assessment
                let risk = risks.iter().find(|r| r.symbol == *symbol);
                let risk_decision = risk.and_then(|r| r.decision.clone());
                let risk_reward_ratio = risk.and_then(|r| r.risk_reward_ratio).or(signal.risk_reward);
                let mut position_size_pct = risk.and_then(|r| r.position_size_pct);

                // Skip REJECTED trades
                if risk_decision.as_deref() == Some("REJECT") {
                    tracing::info!("{}: REJECTED by Risk Manager — skipping", symbol);
                    return None;
                }

                // Get strategy optimizer output
                let strategy = strategies.iter().find(|s| s.symbol == *symbol);
                let mut execution_action = strategy.and_then(|s| s.execution_action.clone());

                // Apply strategy adjustments
                if let Some(strat) = strategy {
                    if strat.execution_action.as_deref() == Some("SKIP_TRADE") {
                        tracing::info!("{}: SKIPPED by Strategy Optimizer", symbol);
                        return None;
                    }
                    if let Some(adj_entry) = strat.adjusted_entry {
                        if adj_entry > 0.0 {
                            tracing::info!("{}: Optimizer adjusted entry {:.2} → {:.2}", symbol, entry_price, adj_entry);
                        }
                    }
                    if let Some(adj_target) = strat.adjusted_target {
                        if adj_target > 0.0 {
                            target_price = adj_target;
                        }
                    }
                    if let Some(adj_stop) = strat.adjusted_stop {
                        if adj_stop > 0.0 {
                            stop_loss = adj_stop;
                        }
                    }
                    if let Some(adj_size) = strat.adjusted_position_size_pct {
                        position_size_pct = Some(adj_size);
                    }
                }

                // Get review (Haiku) verdict
                let signal_idx = signals.iter().position(|s| s.symbol == *symbol)?;
                let review = reviews.get(signal_idx);

                let (mut final_confidence, review_agreed, review_confidence) = match review {
                    Some(rev) => {
                        let verdict = rev.review_result.as_ref();
                        let final_verdict = verdict.and_then(|v| v.final_verdict.as_deref());
                        let final_decision = verdict.and_then(|v| v.final_decision.as_deref());
                        let rev_confidence = verdict.and_then(|v| v.confidence).unwrap_or(confidence);

                        if let Some(decision) = final_decision {
                            if decision == "NO_TRADE" {
                                tracing::info!("{}: NO_TRADE by Review AI finalDecision — skipping", symbol);
                                return None;
                            }

                            let decision_dir = match decision {
                                "LONG" => "LONG",
                                "SHORT" => "SHORT",
                                _ => "",
                            };
                            if !decision_dir.is_empty() && decision_dir != direction {
                                tracing::info!(
                                    "{}: Review AI finalDecision {} conflicts with signal {} — skipping",
                                    symbol, decision, direction
                                );
                                return None;
                            }

                            match final_verdict {
                                Some("REJECT") => {
                                    tracing::info!("{}: REJECTED by Review AI — skipping", symbol);
                                    return None;
                                }
                                Some("DOWNGRADE") => {
                                    let downgraded = (confidence - 20.0).max(20.0);
                                    tracing::info!(
                                        "{}: DOWNGRADED by Review AI: {:.0}% → {:.0}%",
                                        symbol, confidence, downgraded
                                    );
                                    execution_action = Some("REDUCED_SIZE".into());
                                    (downgraded, Some(false), Some(rev_confidence))
                                }
                                Some("ACCEPT_WITH_CAUTION") => {
                                    let cautious = (confidence - 10.0).max(25.0);
                                    tracing::info!(
                                        "{}: ACCEPTED WITH CAUTION by Review AI: {:.0}% → {:.0}%",
                                        symbol, confidence, cautious
                                    );
                                    (cautious, Some(true), Some(rev_confidence))
                                }
                                Some("ACCEPT") => {
                                    let merged = (confidence + rev_confidence) / 2.0;
                                    tracing::info!(
                                        "{}: ACCEPTED by Review AI. Signal {:.0}% + Review {:.0}% → {:.0}%",
                                        symbol, confidence, rev_confidence, merged
                                    );
                                    (merged, Some(true), Some(rev_confidence))
                                }
                                _ => (confidence, None, None),
                            }
                        } else {
                            match final_verdict {
                                Some("REJECT") => {
                                    tracing::info!("{}: REJECTED by Review AI — skipping", symbol);
                                    return None;
                                }
                                Some("DOWNGRADE") => {
                                    let downgraded = (confidence - 20.0).max(20.0);
                                    tracing::info!(
                                        "{}: DOWNGRADED by Review AI: {:.0}% → {:.0}%",
                                        symbol, confidence, downgraded
                                    );
                                    execution_action = Some("REDUCED_SIZE".into());
                                    (downgraded, Some(false), Some(rev_confidence))
                                }
                                Some("ACCEPT_WITH_CAUTION") => {
                                    let cautious = (confidence - 10.0).max(25.0);
                                    tracing::info!(
                                        "{}: ACCEPTED WITH CAUTION by Review AI: {:.0}% → {:.0}%",
                                        symbol, confidence, cautious
                                    );
                                    (cautious, Some(true), Some(rev_confidence))
                                }
                                Some("ACCEPT") => {
                                    let merged = (confidence + rev_confidence) / 2.0;
                                    tracing::info!(
                                        "{}: ACCEPTED by Review AI. Signal {:.0}% + Review {:.0}% → {:.0}%",
                                        symbol, confidence, rev_confidence, merged
                                    );
                                    (merged, Some(true), Some(rev_confidence))
                                }
                                _ => (confidence, None, None),
                            }
                        }
                    }
                    None => (confidence, None, None),
                };

                // Validate target/stop direction
                if direction == "SHORT" && target_price > entry_price {
                    tracing::warn!("{}: Correcting SHORT target/stop", symbol);
                    std::mem::swap(&mut target_price, &mut stop_loss);
                } else if direction == "LONG" && target_price < entry_price {
                    tracing::warn!("{}: Correcting LONG target/stop", symbol);
                    std::mem::swap(&mut target_price, &mut stop_loss);
                }

                // Clamp targets to max % for the timeframe
                let max_target_pct = match timeframe {
                    "5min"  => 0.0010,
                    "30min" => 0.0030,
                    "1h"    => 0.0070,
                    "6h"    => 0.0150,
                    "12h"   => 0.0250,
                    "24h"   => 0.0400,
                    _       => 0.0080,
                };
                let target_dist = ((target_price - entry_price) / entry_price).abs();
                if target_dist > max_target_pct {
                    tracing::warn!(
                        "{}: Clamping target from {:.4}% to {:.4}% for {}",
                        symbol, target_dist * 100.0, max_target_pct * 100.0, timeframe
                    );
                    target_price = if direction == "LONG" {
                        entry_price * (1.0 + max_target_pct)
                    } else {
                        entry_price * (1.0 - max_target_pct)
                    };
                    let stop_dist = ((stop_loss - entry_price) / entry_price).abs();
                    if stop_dist > max_target_pct {
                        stop_loss = if direction == "LONG" {
                            entry_price * (1.0 - max_target_pct * 0.5)
                        } else {
                            entry_price * (1.0 + max_target_pct * 0.5)
                        };
                    }
                }

                // Cap confidence
                final_confidence = final_confidence.clamp(10.0, 95.0);

                // Extract review details
                let review_verdict_str = review
                    .and_then(|r| r.review_result.as_ref())
                    .and_then(|v| v.final_verdict.clone());
                let review_decision_str = review
                    .and_then(|r| r.review_result.as_ref())
                    .and_then(|v| v.final_decision.clone());
                let review_issues: Vec<String> = review
                    .and_then(|r| r.detected_issues.as_ref())
                    .map(|issues| {
                        issues.iter().filter_map(|di| {
                            let source = di.source.as_deref().unwrap_or("Unknown");
                            let issue = di.issue.as_deref()?;
                            Some(format!("{}: {}", source, issue))
                        }).collect()
                    })
                    .unwrap_or_default();
                let review_notes_vec: Vec<String> = review
                    .and_then(|r| r.review_notes.clone())
                    .unwrap_or_default();

                // Build reasoning with pipeline context
                let risk_notes = risk.and_then(|r| r.risk_notes.as_deref()).unwrap_or("");
                let exec_notes = strategy.and_then(|s| s.execution_notes.as_deref()).unwrap_or("");
                let review_notes_joined = review_notes_vec.join("; ");

                let full_reasoning = format!(
                    "{reasoning}\n\n\
                    [Risk: {risk_decision} | R:R {rr:.1}:1] {risk_notes}\n\
                    [Execution: {exec_action}] {exec_notes}\n\
                    {review_section}",
                    reasoning = reasoning,
                    risk_decision = risk_decision.as_deref().unwrap_or("N/A"),
                    rr = risk_reward_ratio.unwrap_or(0.0),
                    risk_notes = risk_notes,
                    exec_action = execution_action.as_deref().unwrap_or("N/A"),
                    exec_notes = exec_notes,
                    review_section = if review_notes_joined.is_empty() {
                        String::new()
                    } else {
                        format!("[Review: {}]", review_notes_joined)
                    },
                );

                let review_issues_opt = if review_issues.is_empty() { None } else { Some(review_issues) };
                // Derive prediction_reason before moving review_notes_vec
                let first_review_note = review_notes_vec.first().cloned();

                let review_notes_opt = if review_notes_vec.is_empty() { None } else { Some(review_notes_vec) };

                // Derive unified prediction status
                let prediction_status: Option<String> = match (review_verdict_str.as_deref(), execution_action.as_deref()) {
                    (Some("REJECT"), _) => Some("REJECTED".into()),
                    (Some("DOWNGRADE"), _) => Some("DOWNGRADED".into()),
                    (Some("ACCEPT_WITH_CAUTION"), _) => Some("ACCEPT_WITH_CAUTION".into()),
                    (_, Some("WAIT_CONFIRMATION")) => Some("WAIT_CONFIRMATION".into()),
                    (_, Some("REDUCED_SIZE")) => Some("REDUCED_SIZE".into()),
                    (Some("ACCEPT"), _) => Some("APPROVED".into()),
                    _ => Some("APPROVED".into()),
                };

                // Derive prediction_reason from status
                let prediction_reason: Option<String> = match prediction_status.as_deref() {
                    Some("REJECTED") => Some(
                        first_review_note
                            .unwrap_or_else(|| "Trade rejected by review".into())
                    ),
                    Some("DOWNGRADED") => Some("Confidence reduced due to identified risks".into()),
                    Some("WAIT_CONFIRMATION") => Some("Setup needs further confirmation".into()),
                    Some("REDUCED_SIZE") => Some("Position size reduced due to elevated risk".into()),
                    Some("ACCEPT_WITH_CAUTION") => Some("Valid setup with moderate risk factors".into()),
                    Some("APPROVED") => Some("Setup approved with aligned indicators".into()),
                    _ => None,
                };

                let prediction = Prediction::new(
                    symbol,
                    direction,
                    final_confidence,
                    &full_reasoning,
                    entry_price,
                    target_price,
                    stop_loss,
                    None,
                    None,
                    None,
                    None,
                    Some(timeframe.to_string()),
                )
                .with_pipeline(
                    market_bias,
                    setup_type,
                    risk_decision,
                    risk_reward_ratio,
                    execution_action,
                    secondary_target,
                    invalidation,
                    position_size_pct,
                    review_agreed,
                    review_confidence,
                    review_verdict_str,
                    review_decision_str,
                    review_issues_opt,
                    review_notes_opt,
                    trend_strength,
                    momentum,
                    volume_profile,
                    derivatives_sentiment,
                    prediction_status,
                    market_signals,
                    prediction_reason,
                    signal.confluence_score,
                    signal.issues.clone(),
                );

                tracing::info!(
                    "{}: {} {:.0}% | {} | {}",
                    symbol,
                    direction,
                    final_confidence,
                    prediction.get_setup_type().unwrap_or("N/A"),
                    prediction.get_execution_action().unwrap_or("N/A"),
                );

                Some(prediction)
            })
            .collect()
    }
}

fn compute_rsi_for_features(closes: &[f64], period: usize) -> f64 {
    if closes.len() <= period {
        return 50.0;
    }
    let mut avg_gain = 0.0;
    let mut avg_loss = 0.0;
    for i in 1..=period {
        let change = closes[i] - closes[i - 1];
        if change > 0.0 {
            avg_gain += change;
        } else {
            avg_loss += change.abs();
        }
    }
    avg_gain /= period as f64;
    avg_loss /= period as f64;
    for i in (period + 1)..closes.len() {
        let change = closes[i] - closes[i - 1];
        if change > 0.0 {
            avg_gain = (avg_gain * (period as f64 - 1.0) + change) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0)) / period as f64;
        } else {
            avg_gain = (avg_gain * (period as f64 - 1.0)) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0) + change.abs()) / period as f64;
        }
    }
    if avg_loss == 0.0 {
        return 100.0;
    }
    let rs = avg_gain / avg_loss;
    100.0 - (100.0 / (1.0 + rs))
}
