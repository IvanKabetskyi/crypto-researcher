import apiClient from 'api/client';
import { transformPredictionResponse, transformHistoryResponse } from 'api/mappers';
import { Prediction } from 'types/prediction';
import { HistoryResponse, HistoryParams } from 'types/history';
import { AppConfig } from 'types/config';
import { LoginParams, LoginResponse } from 'types/auth';

export const authRequests = {
    login: async (params: LoginParams): Promise<LoginResponse> => {
        const response = await apiClient.post('/auth/login', params);
        return response.data;
    },
};

export const configRequests = {
    fetchConfig: async (): Promise<AppConfig> => {
        const response = await apiClient.get('/config');
        return response.data;
    },
};

export const predictionRequests = {
    triggerAnalysis: async (params: {
        pairs: string[];
        timeframe: string;
        min_confidence: number;
    }): Promise<Prediction[]> => {
        const response = await apiClient.post('/analyze', params, { timeout: 300000 });
        return response.data.map(transformPredictionResponse);
    },

    fetchHistory: async (params?: HistoryParams): Promise<HistoryResponse> => {
        const response = await apiClient.get('/predictions/history', { params });
        return transformHistoryResponse(response.data);
    },
};
