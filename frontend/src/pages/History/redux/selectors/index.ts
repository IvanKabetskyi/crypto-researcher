import { RootState } from 'store/types';

export const getHistoryItems = (state: RootState) => state.history.items;
export const getHistoryTotal = (state: RootState) => state.history.total;
export const getHistoryPage = (state: RootState) => state.history.page;
export const getHistoryPerPage = (state: RootState) => state.history.perPage;
export const getHistoryTotalPages = (state: RootState) => state.history.totalPages;
export const getHistoryLoading = (state: RootState) => state.history.loading;
export const getHistoryError = (state: RootState) => state.history.error;
