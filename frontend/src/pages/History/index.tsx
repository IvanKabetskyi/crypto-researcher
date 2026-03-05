import React, { useEffect, useState, useCallback } from 'react';
import {
    Box,
    Typography,
    CircularProgress,
    Alert,
    Table,
    TableBody,
    TableCell,
    TableContainer,
    TableHead,
    TableRow,
    Paper,
    Chip,
    Select,
    MenuItem,
    FormControl,
    InputLabel,
    TextField,
    Button,
    TablePagination,
} from '@mui/material';
import DownloadIcon from '@mui/icons-material/Download';
import TrendingUpIcon from '@mui/icons-material/TrendingUp';
import TrendingDownIcon from '@mui/icons-material/TrendingDown';
import { useHistoryActions } from './useHistoryActions';
import { useStateHistory } from './useStateHistory';
import { HistoryParams } from 'types/history';

const formatPrice = (price: number) => {
    if (price >= 1) return price.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 });
    return price.toFixed(6);
};

export const History: React.FC = () => {
    const { loadHistory } = useHistoryActions();
    const { items, total, page, perPage, loading, error } = useStateHistory();

    const [symbol, setSymbol] = useState<string>('');
    const [direction, setDirection] = useState<string>('');
    const [outcome, setOutcome] = useState<string>('');
    const [dateFrom, setDateFrom] = useState<string>('');
    const [dateTo, setDateTo] = useState<string>('');

    const buildParams = useCallback(
        (pageOverride?: number): HistoryParams => {
            const params: HistoryParams = {
                page: pageOverride ?? page,
                per_page: perPage,
            };
            if (symbol) params.symbol = symbol;
            if (direction) params.direction = direction;
            if (outcome) params.outcome = outcome;
            if (dateFrom) params.date_from = new Date(dateFrom).toISOString();
            if (dateTo) params.date_to = new Date(dateTo + 'T23:59:59').toISOString();
            return params;
        },
        [symbol, direction, outcome, dateFrom, dateTo, page, perPage],
    );

    useEffect(() => {
        loadHistory(buildParams(1));
    }, []);

    const handleFilter = () => {
        loadHistory(buildParams(1));
    };

    const handlePageChange = (_: unknown, newPage: number) => {
        loadHistory(buildParams(newPage + 1));
    };

    const handleExportCsv = () => {
        if (items.length === 0) return;
        const headers = ['Symbol', 'Direction', 'Confidence', 'Entry Price', 'Target Price', 'Stop Loss', 'Outcome', 'Created At'];
        const rows = items.map((p) => [
            p.symbol,
            p.direction,
            p.confidence.toFixed(1),
            formatPrice(p.entryPrice),
            formatPrice(p.targetPrice),
            formatPrice(p.stopLoss),
            p.outcome ?? 'pending',
            new Date(p.createdAt).toLocaleString(),
        ]);
        const csv = [headers.join(','), ...rows.map((r) => r.join(','))].join('\n');
        const blob = new Blob([csv], { type: 'text/csv' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `prediction-history-${new Date().toISOString().slice(0, 10)}.csv`;
        a.click();
        URL.revokeObjectURL(url);
    };

    const outcomeColor = (val: string | null) => {
        switch (val) {
            case 'correct':
                return 'success';
            case 'incorrect':
                return 'error';
            default:
                return 'warning';
        }
    };

    return (
        <Box sx={{ p: 3 }}>
            <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
                <Typography variant="h4">Prediction History</Typography>
                <Button
                    variant="outlined"
                    startIcon={<DownloadIcon />}
                    onClick={handleExportCsv}
                    disabled={items.length === 0}
                >
                    Export CSV
                </Button>
            </Box>

            <Paper sx={{ p: 2, mb: 3, backgroundColor: 'background.paper' }}>
                <Box display="flex" gap={2} flexWrap="wrap" alignItems="center">
                    <FormControl size="small" sx={{ minWidth: 140 }}>
                        <InputLabel>Symbol</InputLabel>
                        <Select value={symbol} label="Symbol" onChange={(e) => setSymbol(e.target.value)}>
                            <MenuItem value="">All</MenuItem>
                            <MenuItem value="BTCUSDT">BTCUSDT</MenuItem>
                            <MenuItem value="ETHUSDT">ETHUSDT</MenuItem>
                            <MenuItem value="SOLUSDT">SOLUSDT</MenuItem>
                            <MenuItem value="XRPUSDT">XRPUSDT</MenuItem>
                            <MenuItem value="DOGEUSDT">DOGEUSDT</MenuItem>
                        </Select>
                    </FormControl>

                    <FormControl size="small" sx={{ minWidth: 120 }}>
                        <InputLabel>Direction</InputLabel>
                        <Select value={direction} label="Direction" onChange={(e) => setDirection(e.target.value)}>
                            <MenuItem value="">All</MenuItem>
                            <MenuItem value="long">Long</MenuItem>
                            <MenuItem value="short">Short</MenuItem>
                        </Select>
                    </FormControl>

                    <FormControl size="small" sx={{ minWidth: 120 }}>
                        <InputLabel>Outcome</InputLabel>
                        <Select value={outcome} label="Outcome" onChange={(e) => setOutcome(e.target.value)}>
                            <MenuItem value="">All</MenuItem>
                            <MenuItem value="correct">Correct</MenuItem>
                            <MenuItem value="incorrect">Incorrect</MenuItem>
                            <MenuItem value="pending">Pending</MenuItem>
                        </Select>
                    </FormControl>

                    <TextField
                        size="small"
                        type="date"
                        label="From"
                        value={dateFrom}
                        onChange={(e) => setDateFrom(e.target.value)}
                        InputLabelProps={{ shrink: true }}
                    />

                    <TextField
                        size="small"
                        type="date"
                        label="To"
                        value={dateTo}
                        onChange={(e) => setDateTo(e.target.value)}
                        InputLabelProps={{ shrink: true }}
                    />

                    <Button variant="contained" onClick={handleFilter} sx={{ fontWeight: 700 }}>
                        Apply
                    </Button>
                </Box>
            </Paper>

            {error && (
                <Alert severity="error" sx={{ mb: 2 }}>
                    {error}
                </Alert>
            )}

            {loading && items.length === 0 ? (
                <Box display="flex" justifyContent="center" alignItems="center" minHeight={300}>
                    <CircularProgress color="primary" />
                </Box>
            ) : items.length === 0 ? (
                <Box display="flex" justifyContent="center" alignItems="center" minHeight={300}>
                    <Typography color="text.secondary">No predictions found for the selected filters.</Typography>
                </Box>
            ) : (
                <Paper sx={{ backgroundColor: 'background.paper' }}>
                    <TableContainer>
                        <Table size="small">
                            <TableHead>
                                <TableRow>
                                    <TableCell>Symbol</TableCell>
                                    <TableCell>Direction</TableCell>
                                    <TableCell>Timeframe</TableCell>
                                    <TableCell>Confidence</TableCell>
                                    <TableCell>Entry</TableCell>
                                    <TableCell>Target</TableCell>
                                    <TableCell>Stop Loss</TableCell>
                                    <TableCell>Outcome</TableCell>
                                    <TableCell>Date</TableCell>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                {items.map((p) => (
                                    <TableRow key={p.id} hover>
                                        <TableCell>
                                            <Typography fontWeight={600}>{p.symbol}</Typography>
                                        </TableCell>
                                        <TableCell>
                                            <Chip
                                                icon={p.direction === 'long' ? <TrendingUpIcon /> : <TrendingDownIcon />}
                                                label={p.direction.toUpperCase()}
                                                size="small"
                                                color={p.direction === 'long' ? 'success' : 'error'}
                                                variant="outlined"
                                            />
                                        </TableCell>
                                        <TableCell>{p.timeframe || '-'}</TableCell>
                                        <TableCell>{p.confidence.toFixed(1)}%</TableCell>
                                        <TableCell>${formatPrice(p.entryPrice)}</TableCell>
                                        <TableCell>${formatPrice(p.targetPrice)}</TableCell>
                                        <TableCell>${formatPrice(p.stopLoss)}</TableCell>
                                        <TableCell>
                                            <Chip
                                                label={p.outcome ?? 'pending'}
                                                size="small"
                                                color={outcomeColor(p.outcome)}
                                            />
                                        </TableCell>
                                        <TableCell>
                                            <Typography variant="caption" color="text.secondary">
                                                {new Date(p.createdAt).toLocaleString()}
                                            </Typography>
                                        </TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    </TableContainer>
                    <TablePagination
                        component="div"
                        count={total}
                        page={page - 1}
                        onPageChange={handlePageChange}
                        rowsPerPage={perPage}
                        rowsPerPageOptions={[20]}
                    />
                </Paper>
            )}
        </Box>
    );
};
