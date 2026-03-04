import { RootState } from 'store/types';

export const getFilterSymbol = (state: RootState) => state.filter.symbol;
export const getFilterMinConfidence = (state: RootState) => state.filter.minConfidence;
export const getFilterMaxConfidence = (state: RootState) => state.filter.maxConfidence;
export const getFilterDirection = (state: RootState) => state.filter.direction;
export const getFilters = (state: RootState) => state.filter;
