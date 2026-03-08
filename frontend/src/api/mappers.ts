import { Prediction } from 'types/prediction';
import { HistoryResponse } from 'types/history';

interface PredictionApiResponse {
    id: string;
    symbol: string;
    direction: string;
    confidence: number;
    reasoning: string;
    entry_price: number;
    target_price: number;
    stop_loss: number;
    created_at: string;
    outcome: string | null;
    timeframe?: string;
    market_bias?: string;
    setup_type?: string;
    risk_decision?: string;
    risk_reward_ratio?: number;
    execution_action?: string;
    secondary_target?: number;
    invalidation?: number;
    position_size_pct?: number;
    review_agreed?: boolean;
    review_confidence?: number;
    review_verdict?: string;
    review_decision?: string;
    review_issues?: string[];
    review_notes?: string[];
    trend_strength?: string;
    momentum?: string;
    volume_profile?: string;
    derivatives_sentiment?: string;
    prediction_status?: string;
    market_signals?: string[];
    prediction_reason?: string;
}

export const transformPredictionResponse = (data: PredictionApiResponse): Prediction => ({
    id: data.id,
    symbol: data.symbol,
    direction: data.direction as Prediction['direction'],
    confidence: data.confidence,
    reasoning: data.reasoning,
    entryPrice: data.entry_price,
    targetPrice: data.target_price,
    stopLoss: data.stop_loss,
    createdAt: data.created_at,
    outcome: data.outcome,
    timeframe: data.timeframe,
    marketBias: data.market_bias,
    riskDecision: data.risk_decision,
    riskRewardRatio: data.risk_reward_ratio,
    executionAction: data.execution_action,
    positionSizePct: data.position_size_pct,
    reviewAgreed: data.review_agreed,
    reviewConfidence: data.review_confidence,
    reviewVerdict: data.review_verdict,
    reviewDecision: data.review_decision,
    reviewIssues: data.review_issues,
    reviewNotes: data.review_notes,
    trendStrength: data.trend_strength as Prediction['trendStrength'],
    momentum: data.momentum as Prediction['momentum'],
    volumeProfile: data.volume_profile as Prediction['volumeProfile'],
    derivativesSentiment: data.derivatives_sentiment as Prediction['derivativesSentiment'],
    predictionStatus: data.prediction_status,
    marketSignals: data.market_signals,
    predictionReason: data.prediction_reason,
});

interface HistoryApiResponse {
    items: PredictionApiResponse[];
    total: number;
    page: number;
    per_page: number;
    total_pages: number;
}

export const transformHistoryResponse = (data: HistoryApiResponse): HistoryResponse => ({
    items: data.items.map(transformPredictionResponse),
    total: data.total,
    page: data.page,
    perPage: data.per_page,
    totalPages: data.total_pages,
});
