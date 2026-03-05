import { useDispatch } from 'react-redux';
import { AppDispatch } from 'store/types';
import { fetchHistory } from './asyncActions';
import { HistoryParams } from 'types/history';

export const useHistoryActions = () => {
    const dispatch = useDispatch<AppDispatch>();

    return {
        loadHistory: (params?: HistoryParams) => dispatch(fetchHistory(params)),
    };
};
