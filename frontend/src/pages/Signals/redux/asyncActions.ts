import { createAsyncThunk } from '@reduxjs/toolkit';
import { configRequests, predictionRequests } from 'api/requests';
import { Prediction } from 'types/prediction';
import { AppConfig } from 'types/config';

interface RunAnalysisParams {
    pairs: string[];
    timeframe: string;
    min_confidence: number;
    bet_value: number;
    onStage?: (stage: string) => void;
}

export const runAnalysis = createAsyncThunk<Prediction[], RunAnalysisParams>(
    'signals/runAnalysis',
    async (params, { rejectWithValue }) => {
        try {
            const { onStage, ...apiParams } = params;
            return await predictionRequests.triggerAnalysis(apiParams, { onStage });
        } catch (err: unknown) {
            const message = (err as Error)?.message
                || (err as { response?: { data?: { error?: string } } })?.response?.data?.error
                || 'Analysis failed';
            return rejectWithValue(message);
        }
    },
);

export const fetchConfig = createAsyncThunk<AppConfig>(
    'signals/fetchConfig',
    async () => {
        return await configRequests.fetchConfig();
    },
);
