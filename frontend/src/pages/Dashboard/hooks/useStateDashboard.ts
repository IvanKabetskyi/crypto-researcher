import { useSelector } from 'react-redux';
import {
    getPredictions,
    getMarketData,
    getDashboardLoading,
    getDashboardError,
    getLastUpdated,
} from '../redux/selectors';

export const useStateDashboard = () => {
    const predictions = useSelector(getPredictions);
    const market = useSelector(getMarketData);
    const loading = useSelector(getDashboardLoading);
    const error = useSelector(getDashboardError);
    const lastUpdated = useSelector(getLastUpdated);

    return { predictions, market, loading, error, lastUpdated };
};
