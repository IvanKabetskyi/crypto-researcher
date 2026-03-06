import { createSlice } from '@reduxjs/toolkit';
import { Prediction } from 'types/prediction';
import { Timeframe } from 'types/config';
import { runAnalysis, fetchConfig } from './asyncActions';

interface SignalsState {
    predictions: Prediction[];
    analyzing: boolean;
    error: string | null;
    availablePairs: string[];
    availableTimeframes: Timeframe[];
    configLoaded: boolean;
}

const initialState: SignalsState = {
    predictions: [],
    analyzing: false,
    error: null,
    availablePairs: [],
    availableTimeframes: [],
    configLoaded: false,
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
                state.error = (action.payload as string) || action.error.message || 'Analysis failed';
            })
            .addCase(fetchConfig.fulfilled, (state, action) => {
                state.availablePairs = action.payload.pairs;
                state.availableTimeframes = action.payload.timeframes;
                state.configLoaded = true;
            });
    },
});

export default signalsSlice.reducer;
