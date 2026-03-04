import { createSlice, PayloadAction } from '@reduxjs/toolkit';

interface FilterState {
    symbol: string | null;
    minConfidence: number;
    maxConfidence: number;
    direction: 'long' | 'short' | null;
}

const initialState: FilterState = {
    symbol: null,
    minConfidence: 80,
    maxConfidence: 90,
    direction: null,
};

const filterSlice = createSlice({
    name: 'filter',
    initialState,
    reducers: {
        setSymbol: (state, action: PayloadAction<string | null>) => {
            state.symbol = action.payload;
        },
        setMinConfidence: (state, action: PayloadAction<number>) => {
            state.minConfidence = action.payload;
        },
        setMaxConfidence: (state, action: PayloadAction<number>) => {
            state.maxConfidence = action.payload;
        },
        setDirection: (state, action: PayloadAction<'long' | 'short' | null>) => {
            state.direction = action.payload;
        },
        resetFilters: () => initialState,
    },
});

export const { setSymbol, setMinConfidence, setMaxConfidence, setDirection, resetFilters } =
    filterSlice.actions;

export default filterSlice.reducer;
