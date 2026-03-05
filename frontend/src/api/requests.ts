import apiClient from 'api/client';
import { transformPredictionResponse, transformHistoryResponse } from 'api/mappers';
import { Prediction } from 'types/prediction';
import { HistoryResponse, HistoryParams } from 'types/history';

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
