import { createAsyncThunk } from '@reduxjs/toolkit';
import { predictionRequests } from 'api/requests';
import { HistoryResponse, HistoryParams } from 'types/history';

export const fetchHistory = createAsyncThunk<HistoryResponse, HistoryParams | undefined>(
    'history/fetchHistory',
    async (params) => {
        return await predictionRequests.fetchHistory(params);
    },
);
