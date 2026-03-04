import { useDispatch } from 'react-redux';
import { AppDispatch } from 'store/types';
import { fetchPredictions, fetchMarketData } from '../redux/asyncActions';

export const useDashboardActions = () => {
    const dispatch = useDispatch<AppDispatch>();

    return {
        loadPredictions: (params?: {
            symbol?: string;
            min_confidence?: number;
            direction?: string;
        }) => dispatch(fetchPredictions(params)),
        loadMarket: () => dispatch(fetchMarketData()),
    };
};
