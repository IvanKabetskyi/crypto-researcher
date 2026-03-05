import { createSlice } from '@reduxjs/toolkit';
import { Prediction } from 'core/Entities/Prediction/types';
import { runAnalysis, fetchSignalsPredictions } from './asyncActions';

interface SignalsState {
    predictions: Prediction[];
    loading: boolean;
    analyzing: boolean;
    error: string | null;
}

const initialState: SignalsState = {
    predictions: [],
    loading: false,
    analyzing: false,
    error: null,
};

const signalsSlice = createSlice({
    name: 'signals',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(runAnalysis.pending, (state) => {
                state.analyzing = true;
                state.error = null;
            })
            .addCase(runAnalysis.fulfilled, (state, action) => {
                state.analyzing = false;
                state.predictions = action.payload;
            })
            .addCase(runAnalysis.rejected, (state, action) => {
                state.analyzing = false;
                state.error = action.error.message || 'Analysis failed';
            })
            .addCase(fetchSignalsPredictions.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchSignalsPredictions.fulfilled, (state, action) => {
                state.loading = false;
                state.predictions = action.payload;
            })
            .addCase(fetchSignalsPredictions.rejected, (state, action) => {
                state.loading = false;
                state.error = action.error.message || 'Failed to fetch predictions';
            });
    },
});

export default signalsSlice.reducer;
