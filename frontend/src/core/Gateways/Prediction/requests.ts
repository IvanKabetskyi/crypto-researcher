import apiClient from 'services/api';
import {
    transformPredictionResponse,
    transformMarketResponse,
    transformAccuracyResponse,
    transformHistoryResponse,
} from './mappers';
import { Prediction } from 'core/Entities/Prediction/types';
import { MarketData } from 'core/Entities/Market/types';
import { AccuracyStats } from 'core/Entities/Accuracy/types';
import { HistoryResponse, HistoryParams } from 'core/Entities/History/types';

interface FetchPredictionsParams {
    symbol?: string;
    min_confidence?: number;
    direction?: string;
    limit?: number;
}

export interface Settings {
    ai_model: string;
    ai_url: string;
    watch_pairs: string;
    analysis_interval_secs: string;
    news_source: string;
}

export interface SettingsPayload {
    ai_model?: string;
    watch_pairs?: string;
    analysis_interval_secs?: number;
}

export const predictionRequests = {
    fetchPredictions: async (params?: FetchPredictionsParams): Promise<Prediction[]> => {
        const response = await apiClient.get('/predictions', { params });
        return response.data.map(transformPredictionResponse);
    },

    fetchAccuracy: async (): Promise<AccuracyStats> => {
        const response = await apiClient.get('/predictions/accuracy');
        return transformAccuracyResponse(response.data);
    },

    fetchMarket: async (): Promise<MarketData[]> => {
        const response = await apiClient.get('/market');
        return response.data.map(transformMarketResponse);
    },

    triggerAnalysis: async (params: {
        pairs: string[];
        timeframe: string;
        min_confidence: number;
    }): Promise<Prediction[]> => {
        const response = await apiClient.post('/analyze', params, { timeout: 300000 });
        return response.data.map(transformPredictionResponse);
    },

    fetchSettings: async (): Promise<Settings> => {
        const response = await apiClient.get('/settings');
        return response.data;
    },

    updateSettings: async (settings: SettingsPayload): Promise<{ success: boolean }> => {
        const response = await apiClient.post('/settings', settings);
        return response.data;
    },

    fetchHistory: async (params?: HistoryParams): Promise<HistoryResponse> => {
        const response = await apiClient.get('/predictions/history', { params });
        return transformHistoryResponse(response.data);
    },
};
