import { useSelector } from 'react-redux';
import {
    getSignalsPredictions,
    getSignalsAnalyzing,
    getSignalsError,
    getAvailablePairs,
    getAvailableTimeframes,
    getConfigLoaded,
} from '../redux/selectors';

export const useStateSignals = () => {
    const predictions = useSelector(getSignalsPredictions);
    const analyzing = useSelector(getSignalsAnalyzing);
    const error = useSelector(getSignalsError);
    const availablePairs = useSelector(getAvailablePairs);
    const availableTimeframes = useSelector(getAvailableTimeframes);
    const configLoaded = useSelector(getConfigLoaded);

    return { predictions, analyzing, error, availablePairs, availableTimeframes, configLoaded };
};
