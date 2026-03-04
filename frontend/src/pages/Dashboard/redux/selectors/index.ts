import { RootState } from 'store/types';
import { selectAllPredictions } from 'pages/Dashboard/redux';

export const getPredictions = (state: RootState) =>
    selectAllPredictions(state.dashboard.predictions);

export const getMarketData = (state: RootState) => state.dashboard.market;

export const getDashboardLoading = (state: RootState) => state.dashboard.loading;

export const getDashboardError = (state: RootState) => state.dashboard.error;

export const getLastUpdated = (state: RootState) => state.dashboard.lastUpdated;
