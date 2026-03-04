import { createSlice, createEntityAdapter } from '@reduxjs/toolkit';
import { Prediction } from 'core/Entities/Prediction/types';
import { MarketData } from 'core/Entities/Market/types';
import { fetchPredictions, fetchMarketData } from './asyncActions';

const predictionsAdapter = createEntityAdapter<Prediction>();

interface DashboardState {
    predictions: ReturnType<typeof predictionsAdapter.getInitialState>;
    market: MarketData[];
    loading: boolean;
    error: string | null;
    lastUpdated: string | null;
}

const initialState: DashboardState = {
    predictions: predictionsAdapter.getInitialState(),
    market: [],
    loading: false,
    error: null,
    lastUpdated: null,
};

const dashboardSlice = createSlice({
    name: 'dashboard',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(fetchPredictions.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchPredictions.fulfilled, (state, action) => {
                state.loading = false;
                predictionsAdapter.setAll(state.predictions, action.payload);
                state.lastUpdated = new Date().toISOString();
            })
            .addCase(fetchPredictions.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to fetch predictions';
            })
            .addCase(fetchMarketData.fulfilled, (state, action) => {
                state.market = action.payload;
            });
    },
});

export default dashboardSlice.reducer;

export const { selectAll: selectAllPredictions } = predictionsAdapter.getSelectors();
