import React from 'react';
import {
    Box,
    FormControl,
    InputLabel,
    Select,
    MenuItem,
    Slider,
    Typography,
    Button,
    SelectChangeEvent,
} from '@mui/material';
import { useDispatch, useSelector } from 'react-redux';
import { AppDispatch } from 'store/types';
import { getFilters } from 'store/slices/Filter/selectors';
import {
    setSymbol,
    setMinConfidence,
    setMaxConfidence,
    setDirection,
    resetFilters,
} from 'store/slices/Filter';

const SYMBOLS = ['BTCUSDT', 'ETHUSDT', 'SOLUSDT', 'XRPUSDT', 'DOGEUSDT'];

export const FilterPanel: React.FC = () => {
    const dispatch = useDispatch<AppDispatch>();
    const filters = useSelector(getFilters);

    const handleSymbolChange = (event: SelectChangeEvent<string>) => {
        const value = event.target.value;
        dispatch(setSymbol(value === 'all' ? null : value));
    };

    const handleDirectionChange = (event: SelectChangeEvent<string>) => {
        const value = event.target.value;
        dispatch(setDirection(value === 'all' ? null : (value as 'long' | 'short')));
    };

    const handleConfidenceChange = (_: Event, newValue: number | number[]) => {
        const [min, max] = newValue as number[];
        dispatch(setMinConfidence(min));
        dispatch(setMaxConfidence(max));
    };

    return (
        <Box
            sx={{
                display: 'flex',
                gap: 2,
                alignItems: 'center',
                flexWrap: 'wrap',
                p: 2,
                backgroundColor: 'background.paper',
                borderRadius: 2,
                border: '1px solid rgba(255,255,255,0.06)',
            }}
        >
            <FormControl size="small" sx={{ minWidth: 140 }}>
                <InputLabel>Symbol</InputLabel>
                <Select
                    value={filters.symbol || 'all'}
                    label="Symbol"
                    onChange={handleSymbolChange}
                >
                    <MenuItem value="all">All Pairs</MenuItem>
                    {SYMBOLS.map((s) => (
                        <MenuItem key={s} value={s}>
                            {s}
                        </MenuItem>
                    ))}
                </Select>
            </FormControl>

            <FormControl size="small" sx={{ minWidth: 120 }}>
                <InputLabel>Direction</InputLabel>
                <Select
                    value={filters.direction || 'all'}
                    label="Direction"
                    onChange={handleDirectionChange}
                >
                    <MenuItem value="all">All</MenuItem>
                    <MenuItem value="long">Long</MenuItem>
                    <MenuItem value="short">Short</MenuItem>
                </Select>
            </FormControl>

            <Box sx={{ minWidth: 200, px: 1 }}>
                <Typography variant="caption" color="text.secondary">
                    Confidence: {filters.minConfidence}% - {filters.maxConfidence}%
                </Typography>
                <Slider
                    value={[filters.minConfidence, filters.maxConfidence]}
                    onChange={handleConfidenceChange}
                    min={50}
                    max={100}
                    size="small"
                    valueLabelDisplay="auto"
                    sx={{ color: 'primary.main' }}
                />
            </Box>

            <Button variant="outlined" size="small" onClick={() => dispatch(resetFilters())}>
                Reset
            </Button>
        </Box>
    );
};
