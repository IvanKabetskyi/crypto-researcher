import { Prediction } from 'core/Entities/Prediction/types';
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
    timeframe?: string;
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
    outcome: data.outcome,
    timeframe: data.timeframe,
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
