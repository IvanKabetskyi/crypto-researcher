import { useDispatch } from 'react-redux';
import { AppDispatch } from 'store/types';
import { runAnalysis, fetchSignalsPredictions } from '../redux/asyncActions';

export const useSignalsActions = () => {
    const dispatch = useDispatch<AppDispatch>();

    return {
        runAnalysis: (params: { pairs: string[]; timeframe: string; min_confidence: number }) =>
            dispatch(runAnalysis(params)),
        loadPredictions: (params?: { min_confidence?: number }) =>
            dispatch(fetchSignalsPredictions(params)),
    };
};
