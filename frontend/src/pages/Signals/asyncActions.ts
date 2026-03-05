import { createAsyncThunk } from '@reduxjs/toolkit';
import { predictionRequests } from 'api/requests';
import { Prediction } from 'types/prediction';

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
