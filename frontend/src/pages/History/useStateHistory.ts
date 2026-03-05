import { useSelector } from 'react-redux';
import {
    getHistoryItems,
    getHistoryTotal,
    getHistoryPage,
    getHistoryPerPage,
    getHistoryTotalPages,
    getHistoryLoading,
    getHistoryError,
} from './selectors';

export const useStateHistory = () => {
    const items = useSelector(getHistoryItems);
    const total = useSelector(getHistoryTotal);
    const page = useSelector(getHistoryPage);
    const perPage = useSelector(getHistoryPerPage);
    const totalPages = useSelector(getHistoryTotalPages);
    const loading = useSelector(getHistoryLoading);
    const error = useSelector(getHistoryError);

    return { items, total, page, perPage, totalPages, loading, error };
};
