import { useDispatch } from 'react-redux';
import { AppDispatch } from 'store/types';
import { fetchHistory } from '../redux/asyncActions';
import { HistoryParams } from 'core/Entities/History/types';

export const useHistoryActions = () => {
    const dispatch = useDispatch<AppDispatch>();

    return {
        loadHistory: (params?: HistoryParams) => dispatch(fetchHistory(params)),
    };
};
