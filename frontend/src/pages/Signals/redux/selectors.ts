import { RootState } from 'store/types';

export const getSignalsPredictions = (state: RootState) => state.signals.predictions;
export const getSignalsAnalyzing = (state: RootState) => state.signals.analyzing;
export const getSignalsError = (state: RootState) => state.signals.error;
export const getAvailablePairs = (state: RootState) => state.signals.availablePairs;
export const getAvailableTimeframes = (state: RootState) => state.signals.availableTimeframes;
export const getConfigLoaded = (state: RootState) => state.signals.configLoaded;
