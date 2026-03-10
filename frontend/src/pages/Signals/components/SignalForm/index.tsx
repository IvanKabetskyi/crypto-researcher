import React from 'react';
import {
    Box,
    Button,
    Chip,
    CircularProgress,
    FormControl,
    FormHelperText,
    InputLabel,
    MenuItem,
    Select,
    TextField,
    Typography,
    Paper,
} from '@mui/material';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { Timeframe } from 'types/config';

interface SignalFormProps {
    onSubmit: (params: { pairs: string[]; timeframe: string; min_confidence: number; bet_value: number }) => void;
    analyzing: boolean;
    directionFilter: string;
    onDirectionFilterChange: (value: string) => void;
    availablePairs: string[];
    availableTimeframes: Timeframe[];
}

export const SignalForm: React.FC<SignalFormProps> = ({
    onSubmit,
    analyzing,
    directionFilter,
    onDirectionFilterChange,
    availablePairs,
    availableTimeframes,
}) => {
    const [selectedPairs, setSelectedPairs] = React.useState<string[]>([]);
    const [timeframe, setTimeframe] = React.useState('');
    const [initialized, setInitialized] = React.useState(false);

    React.useEffect(() => {
        if (!initialized && availablePairs.length > 0 && availableTimeframes.length > 0) {
            setSelectedPairs(availablePairs.slice(0, 2));
            setTimeframe(availableTimeframes[1]?.value ?? availableTimeframes[0].value);
            setInitialized(true);
        }
    }, [availablePairs, availableTimeframes, initialized]);
    const [minConfidence, setMinConfidence] = React.useState(30);
    const [betValue, setBetValue] = React.useState(100);

    const togglePair = (pair: string) => {
        setSelectedPairs((prev) =>
            prev.includes(pair) ? prev.filter((p) => p !== pair) : [...prev, pair],
        );
    };

    const handleSubmit = () => {
        if (selectedPairs.length === 0 || betValue <= 0) return;
        onSubmit({
            pairs: selectedPairs,
            timeframe,
            min_confidence: minConfidence,
            bet_value: betValue,
        });
    };

    return (
        <Paper sx={{ p: 2.5, mb: 3, backgroundColor: 'background.paper' }}>
            <Box mb={2}>
                <Typography variant="subtitle2" color="text.secondary" mb={1}>
                    Trading Pairs
                </Typography>
                <Box display="flex" gap={1} flexWrap="wrap">
                    {availablePairs.map((pair) => (
                        <Chip
                            key={pair}
                            label={pair}
                            onClick={() => togglePair(pair)}
                            color={selectedPairs.includes(pair) ? 'primary' : 'default'}
                            variant={selectedPairs.includes(pair) ? 'filled' : 'outlined'}
                            sx={{ fontWeight: 600 }}
                        />
                    ))}
                </Box>
            </Box>

            <Box display="flex" gap={2} flexWrap="wrap" alignItems="center">
                <FormControl size="small" sx={{ minWidth: 120 }}>
                    <InputLabel>Timeframe</InputLabel>
                    <Select
                        value={timeframe}
                        label="Timeframe"
                        onChange={(e) => setTimeframe(e.target.value)}
                    >
                        {availableTimeframes.map((tf) => (
                            <MenuItem key={tf.value} value={tf.value}>
                                {tf.label}
                            </MenuItem>
                        ))}
                    </Select>
                    {timeframe && (
                        <FormHelperText sx={{ mx: 0, mt: 0.5, color: 'text.secondary', fontSize: '0.7rem' }}>
                            {availableTimeframes.find((tf) => tf.value === timeframe)?.description}
                        </FormHelperText>
                    )}
                </FormControl>

                <TextField
                    size="small"
                    type="number"
                    label="Bet Value ($)"
                    value={betValue}
                    onChange={(e) => {
                        const val = Math.max(1, Number(e.target.value));
                        setBetValue(val);
                    }}
                    inputProps={{ min: 1, step: 10 }}
                    required
                    sx={{ width: 140 }}
                />

                <TextField
                    size="small"
                    type="number"
                    label="Min Confidence"
                    value={minConfidence}
                    onChange={(e) => {
                        const val = Math.max(0, Math.min(100, Number(e.target.value)));
                        setMinConfidence(val);
                    }}
                    inputProps={{ min: 0, max: 100 }}
                    sx={{ width: 140 }}
                />

                <FormControl size="small" sx={{ minWidth: 120 }}>
                    <InputLabel>Direction</InputLabel>
                    <Select
                        value={directionFilter}
                        label="Direction"
                        onChange={(e) => onDirectionFilterChange(e.target.value)}
                    >
                        <MenuItem value="all">All</MenuItem>
                        <MenuItem value="long">Long</MenuItem>
                        <MenuItem value="short">Short</MenuItem>
                    </Select>
                </FormControl>

                <Button
                    variant="contained"
                    startIcon={
                        analyzing ? (
                            <CircularProgress size={18} color="inherit" />
                        ) : (
                            <PlayArrowIcon />
                        )
                    }
                    onClick={handleSubmit}
                    disabled={analyzing || selectedPairs.length === 0}
                    sx={{ fontWeight: 700, height: 40 }}
                >
                    {analyzing ? 'Analyzing...' : 'Run Analysis'}
                </Button>
            </Box>
        </Paper>
    );
};
