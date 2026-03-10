import React from 'react';
import {
    Box,
    Button,
    Chip,
    CircularProgress,
    FormControl,
    InputLabel,
    MenuItem,
    Select,
    TextField,
    Paper,
    Tooltip,
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

    const selectedTf = availableTimeframes.find((tf) => tf.value === timeframe);

    return (
        <Paper sx={{ p: 1.5, mb: 2, backgroundColor: 'background.paper' }}>
            <Box display="flex" gap={1} alignItems="center" flexWrap="wrap">
                {availablePairs.map((pair) => (
                    <Chip
                        key={pair}
                        label={pair}
                        onClick={() => togglePair(pair)}
                        color={selectedPairs.includes(pair) ? 'primary' : 'default'}
                        variant={selectedPairs.includes(pair) ? 'filled' : 'outlined'}
                        size="small"
                        sx={{ fontWeight: 600 }}
                    />
                ))}

                <Box sx={{ mx: 0.5, borderLeft: '1px solid rgba(255,255,255,0.1)', height: 28 }} />

                <Tooltip title={selectedTf?.description || ''} arrow placement="top">
                    <FormControl size="small" sx={{ minWidth: 90 }}>
                        <InputLabel>TF</InputLabel>
                        <Select
                            value={timeframe}
                            label="TF"
                            onChange={(e) => setTimeframe(e.target.value)}
                            sx={{ fontSize: '0.85rem' }}
                        >
                            {availableTimeframes.map((tf) => (
                                <MenuItem key={tf.value} value={tf.value}>
                                    {tf.label}
                                </MenuItem>
                            ))}
                        </Select>
                    </FormControl>
                </Tooltip>

                <TextField
                    size="small"
                    type="number"
                    label="Bet $"
                    value={betValue}
                    onChange={(e) => setBetValue(Math.max(1, Number(e.target.value)))}
                    inputProps={{ min: 1, step: 10 }}
                    sx={{ width: 90 }}
                />

                <TextField
                    size="small"
                    type="number"
                    label="Min %"
                    value={minConfidence}
                    onChange={(e) => setMinConfidence(Math.max(0, Math.min(100, Number(e.target.value))))}
                    inputProps={{ min: 0, max: 100 }}
                    sx={{ width: 80 }}
                />

                <FormControl size="small" sx={{ minWidth: 90 }}>
                    <InputLabel>Dir</InputLabel>
                    <Select
                        value={directionFilter}
                        label="Dir"
                        onChange={(e) => onDirectionFilterChange(e.target.value)}
                        sx={{ fontSize: '0.85rem' }}
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
                            <CircularProgress size={16} color="inherit" />
                        ) : (
                            <PlayArrowIcon />
                        )
                    }
                    onClick={handleSubmit}
                    disabled={analyzing || selectedPairs.length === 0}
                    sx={{ fontWeight: 700, height: 36, minWidth: 120, fontSize: '0.85rem' }}
                >
                    {analyzing ? 'Running...' : 'Analyze'}
                </Button>
            </Box>
        </Paper>
    );
};
