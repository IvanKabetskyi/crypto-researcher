import { createAsyncThunk } from '@reduxjs/toolkit';
import { predictionRequests } from 'core/Gateways/Prediction/requests';
import { Prediction } from 'core/Entities/Prediction/types';
import { MarketData } from 'core/Entities/Market/types';

interface FetchPredictionsParams {
    symbol?: string;
    min_confidence?: number;
    direction?: string;
}

export const fetchPredictions = createAsyncThunk<Prediction[], FetchPredictionsParams | undefined>(
    'dashboard/fetchPredictions',
    async (params) => {
        return await predictionRequests.fetchPredictions(params);
    },
);

export const fetchMarketData = createAsyncThunk<MarketData[]>(
    'dashboard/fetchMarketData',
    async () => {
        return await predictionRequests.fetchMarket();
    },
);
