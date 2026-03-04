import { createAsyncThunk } from '@reduxjs/toolkit';
import { predictionRequests } from 'core/Gateways/Prediction/requests';
import { HistoryResponse, HistoryParams } from 'core/Entities/History/types';

export const fetchHistory = createAsyncThunk<HistoryResponse, HistoryParams | undefined>(
    'history/fetchHistory',
    async (params) => {
        return await predictionRequests.fetchHistory(params);
    },
);
