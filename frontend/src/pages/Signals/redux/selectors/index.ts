import { RootState } from 'store/types';

export const getSignalsPredictions = (state: RootState) => state.signals.predictions;
export const getSignalsAnalyzing = (state: RootState) => state.signals.analyzing;
export const getSignalsError = (state: RootState) => state.signals.error;
