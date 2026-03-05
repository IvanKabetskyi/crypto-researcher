import { useDispatch } from 'react-redux';
import { AppDispatch } from 'store/types';
import { runAnalysis, fetchConfig } from '../redux/asyncActions';

export const useSignalsActions = () => {
    const dispatch = useDispatch<AppDispatch>();

    return {
        runAnalysis: (params: { pairs: string[]; timeframe: string; min_confidence: number }) =>
            dispatch(runAnalysis(params)),
        fetchConfig: () => dispatch(fetchConfig()),
    };
};
