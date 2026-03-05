import { useSelector } from 'react-redux';
import {
    getSignalsPredictions,
    getSignalsLoading,
    getSignalsAnalyzing,
    getSignalsError,
} from '../redux/selectors';

export const useStateSignals = () => {
    const predictions = useSelector(getSignalsPredictions);
    const loading = useSelector(getSignalsLoading);
    const analyzing = useSelector(getSignalsAnalyzing);
    const error = useSelector(getSignalsError);

    return { predictions, loading, analyzing, error };
};
