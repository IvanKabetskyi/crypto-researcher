import { createSlice } from '@reduxjs/toolkit';
import { Prediction } from 'core/Entities/Prediction/types';
import { fetchHistory } from './asyncActions';

interface HistoryState {
    items: Prediction[];
    total: number;
    page: number;
    perPage: number;
    totalPages: number;
    loading: boolean;
    error: string | null;
}

const initialState: HistoryState = {
    items: [],
    total: 0,
    page: 1,
    perPage: 20,
    totalPages: 0,
    loading: false,
    error: null,
};

const historySlice = createSlice({
    name: 'history',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(fetchHistory.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchHistory.fulfilled, (state, action) => {
                state.loading = false;
                state.items = action.payload.items;
                state.total = action.payload.total;
                state.page = action.payload.page;
                state.perPage = action.payload.perPage;
                state.totalPages = action.payload.totalPages;
            })
            .addCase(fetchHistory.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to fetch history';
            });
    },
});

export default historySlice.reducer;
