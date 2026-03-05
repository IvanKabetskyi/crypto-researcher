import apiClient from 'services/api';
import { transformPredictionResponse, transformHistoryResponse } from './mappers';
import { Prediction } from 'core/Entities/Prediction/types';
import { HistoryResponse, HistoryParams } from 'core/Entities/History/types';

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
