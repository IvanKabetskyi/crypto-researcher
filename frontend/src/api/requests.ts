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

interface PollCallbacks {
    onStage?: (stage: string) => void;
}

const pollJob = async (jobId: string, callbacks?: PollCallbacks): Promise<Prediction[]> => {
    const maxAttempts = 150;
    for (let i = 0; i < maxAttempts; i++) {
        await new Promise((r) => setTimeout(r, 2000));
        const res = await apiClient.get(`/analyze/${jobId}`);
        const { status, predictions, error, stage } = res.data;
        if (stage && callbacks?.onStage) {
            callbacks.onStage(stage);
        }
        if (status === 'completed') {
            return (predictions || []).map(transformPredictionResponse);
        }
        if (status === 'failed') {
            throw new Error(error || 'Analysis failed');
        }
    }
    throw new Error('Analysis timed out');
};

export const predictionRequests = {
    triggerAnalysis: async (
        params: { pairs: string[]; timeframe: string; min_confidence: number; bet_value: number },
        callbacks?: PollCallbacks,
    ): Promise<Prediction[]> => {
        const response = await apiClient.post('/analyze', params);
        const jobId = response.data.job_id;
        return pollJob(jobId, callbacks);
    },

    fetchHistory: async (params?: HistoryParams): Promise<HistoryResponse> => {
        const response = await apiClient.get('/predictions/history', { params });
        return transformHistoryResponse(response.data);
    },
};
