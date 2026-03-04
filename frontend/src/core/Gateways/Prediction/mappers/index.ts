import { Prediction } from 'core/Entities/Prediction/types';
import { MarketData } from 'core/Entities/Market/types';
import { AccuracyStats } from 'core/Entities/Accuracy/types';
import { HistoryResponse } from 'core/Entities/History/types';

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
    actual_price_after: number | null;
}

interface MarketApiResponse {
    symbol: string;
    price: number;
    change_24h: number;
    volume_24h: number;
    high_price_24h: number;
    low_price_24h: number;
}

interface AccuracyApiResponse {
    total_predictions: number;
    correct: number;
    incorrect: number;
    pending: number;
    accuracy_percentage: number;
    by_symbol: Record<string, { total: number; correct: number; accuracy: number }>;
}

export const transformPredictionResponse = (data: PredictionApiResponse): Prediction => ({
    id: data.id,
    symbol: data.symbol,
    direction: data.direction as 'long' | 'short',
    confidence: data.confidence,
    reasoning: data.reasoning,
    entryPrice: data.entry_price,
    targetPrice: data.target_price,
    stopLoss: data.stop_loss,
    createdAt: data.created_at,
    outcome: data.outcome as Prediction['outcome'],
    actualPriceAfter: data.actual_price_after,
});

export const transformMarketResponse = (data: MarketApiResponse): MarketData => ({
    symbol: data.symbol,
    price: data.price,
    change24h: data.change_24h,
    volume24h: data.volume_24h,
    highPrice24h: data.high_price_24h,
    lowPrice24h: data.low_price_24h,
});

export const transformAccuracyResponse = (data: AccuracyApiResponse): AccuracyStats => ({
    totalPredictions: data.total_predictions,
    correct: data.correct,
    incorrect: data.incorrect,
    pending: data.pending,
    accuracyPercentage: data.accuracy_percentage,
    bySymbol: data.by_symbol,
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
