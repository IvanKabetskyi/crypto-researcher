import { useSelector } from 'react-redux';
import { getSignalsPredictions, getSignalsAnalyzing, getSignalsError } from './selectors';

export const useStateSignals = () => {
    const predictions = useSelector(getSignalsPredictions);
    const analyzing = useSelector(getSignalsAnalyzing);
    const error = useSelector(getSignalsError);

    return { predictions, analyzing, error };
};
