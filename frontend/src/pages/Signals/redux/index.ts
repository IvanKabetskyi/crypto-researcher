import { createSlice } from '@reduxjs/toolkit';
import { Prediction } from 'core/Entities/Prediction/types';
import { runAnalysis } from './asyncActions';

interface SignalsState {
    predictions: Prediction[];
    analyzing: boolean;
    error: string | null;
}

const initialState: SignalsState = {
    predictions: [],
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
            });
    },
});

export default signalsSlice.reducer;
