import { createSlice } from '@reduxjs/toolkit';
import { AccuracyStats } from 'core/Entities/Accuracy/types';
import { fetchAccuracy } from './asyncActions';

interface AnalyticsState {
    accuracy: AccuracyStats | null;
    loading: boolean;
    error: string | null;
}

const initialState: AnalyticsState = {
    accuracy: null,
    loading: false,
    error: null,
};

const analyticsSlice = createSlice({
    name: 'analytics',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(fetchAccuracy.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchAccuracy.fulfilled, (state, action) => {
                state.loading = false;
                state.accuracy = action.payload;
            })
            .addCase(fetchAccuracy.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to fetch accuracy';
            });
    },
});

export default analyticsSlice.reducer;
