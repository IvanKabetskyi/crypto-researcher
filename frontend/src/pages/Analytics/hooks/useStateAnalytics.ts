import { useSelector } from 'react-redux';
import { getAccuracy, getAnalyticsLoading, getAnalyticsError } from '../redux/selectors';

export const useStateAnalytics = () => {
    const accuracy = useSelector(getAccuracy);
    const loading = useSelector(getAnalyticsLoading);
    const error = useSelector(getAnalyticsError);

    return { accuracy, loading, error };
};
