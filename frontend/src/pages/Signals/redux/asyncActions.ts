import { createAsyncThunk } from '@reduxjs/toolkit';
import { configRequests, predictionRequests } from 'api/requests';
import { Prediction } from 'types/prediction';
import { AppConfig } from 'types/config';

interface RunAnalysisParams {
    pairs: string[];
    timeframe: string;
    min_confidence: number;
}

export const runAnalysis = createAsyncThunk<Prediction[], RunAnalysisParams>(
    'signals/runAnalysis',
    async (params, { rejectWithValue }) => {
        try {
            return await predictionRequests.triggerAnalysis(params);
        } catch (err: unknown) {
            const axiosErr = err as { response?: { data?: { error?: string } } };
            const message = axiosErr?.response?.data?.error || 'Analysis failed';
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
