import apiClient from 'api/client';
import { transformPredictionResponse, transformHistoryResponse } from 'api/mappers';
import { Prediction } from 'types/prediction';
import { HistoryResponse, HistoryParams } from 'types/history';
import { AppConfig } from 'types/config';
import { LoginParams, LoginResponse } from 'types/auth';

declare const __API_URL__: string;

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

interface StreamCallbacks {
    onStage?: (stage: string) => void;
}

export const predictionRequests = {
    triggerAnalysis: async (
        params: { pairs: string[]; timeframe: string; min_confidence: number; bet_value: number },
        callbacks?: StreamCallbacks,
    ): Promise<Prediction[]> => {
        const token = localStorage.getItem('token');
        const response = await fetch(`${__API_URL__}/analyze`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                ...(token ? { Authorization: `Bearer ${token}` } : {}),
            },
            body: JSON.stringify(params),
        });

        if (!response.ok) {
            if (response.status === 401) {
                localStorage.removeItem('token');
                localStorage.removeItem('email');
                window.location.href = import.meta.env.BASE_URL + 'login';
            }
            const text = await response.text().catch(() => response.statusText);
            throw new Error(text || 'Analysis failed');
        }

        const reader = response.body!.getReader();
        const decoder = new TextDecoder();
        let buffer = '';

        while (true) {
            const { done, value } = await reader.read();
            if (done) break;

            buffer += decoder.decode(value, { stream: true });
            const parts = buffer.split('\n\n');
            buffer = parts.pop()!;

            for (const part of parts) {
                const trimmed = part.trim();
                if (!trimmed || trimmed.startsWith(':')) continue;

                let eventType = '';
                let data = '';
                for (const line of trimmed.split('\n')) {
                    if (line.startsWith('event: ')) eventType = line.slice(7);
                    else if (line.startsWith('data: ')) data = line.slice(6);
                }

                if (eventType === 'stage' && data) {
                    callbacks?.onStage?.(data);
                } else if (eventType === 'result' && data) {
                    const predictions = JSON.parse(data);
                    return predictions.map(transformPredictionResponse);
                } else if (eventType === 'error' && data) {
                    throw new Error(data);
                }
            }
        }

        throw new Error('Stream ended without result');
    },

    fetchHistory: async (params?: HistoryParams): Promise<HistoryResponse> => {
        const response = await apiClient.get('/predictions/history', { params });
        return transformHistoryResponse(response.data);
    },
};
