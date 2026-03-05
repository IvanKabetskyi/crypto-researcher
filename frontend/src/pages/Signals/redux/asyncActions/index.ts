import { createAsyncThunk } from '@reduxjs/toolkit';
import { predictionRequests } from 'core/Gateways/Prediction/requests';
import { Prediction } from 'core/Entities/Prediction/types';

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

export const fetchSignalsPredictions = createAsyncThunk<Prediction[], { min_confidence?: number } | undefined>(
    'signals/fetchPredictions',
    async (params) => {
        return await predictionRequests.fetchPredictions({
            min_confidence: params?.min_confidence,
            limit: 50,
        });
    },
);
