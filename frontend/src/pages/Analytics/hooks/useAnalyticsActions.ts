import { useDispatch } from 'react-redux';
import { AppDispatch } from 'store/types';
import { fetchAccuracy } from '../redux/asyncActions';

export const useAnalyticsActions = () => {
    const dispatch = useDispatch<AppDispatch>();

    return {
        loadAccuracy: () => dispatch(fetchAccuracy()),
    };
};
