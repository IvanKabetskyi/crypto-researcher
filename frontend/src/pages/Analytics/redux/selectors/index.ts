import { RootState } from 'store/types';

export const getAccuracy = (state: RootState) => state.analytics.accuracy;
export const getAnalyticsLoading = (state: RootState) => state.analytics.loading;
export const getAnalyticsError = (state: RootState) => state.analytics.error;
