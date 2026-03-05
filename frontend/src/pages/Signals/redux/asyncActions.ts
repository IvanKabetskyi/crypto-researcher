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
    async (params) => {
        return await predictionRequests.triggerAnalysis(params);
    },
);

export const fetchConfig = createAsyncThunk<AppConfig>(
    'signals/fetchConfig',
    async () => {
        return await configRequests.fetchConfig();
    },
);
