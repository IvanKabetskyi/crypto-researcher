import React, { useEffect, useState } from 'react';
import {
    Box,
    Typography,
    Grid,
    CircularProgress,
    Alert,
    Card,
    CardContent,
    TextField,
    Button,
    Snackbar,
    MenuItem,
} from '@mui/material';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { useAnalyticsActions } from './hooks/useAnalyticsActions';
import { useStateAnalytics } from './hooks/useStateAnalytics';
import { StatsCard } from './components/StatsCard';
import { AccuracyChart } from './components/AccuracyChart';
import { PredictionCard } from 'pages/Dashboard/components/PredictionCard';
import { predictionRequests } from 'core/Gateways/Prediction/requests';
import { Prediction } from 'core/Entities/Prediction/types';

const SYMBOLS = ['BTCUSDT', 'ETHUSDT', 'SOLUSDT', 'BNBUSDT', 'XRPUSDT', 'DOGEUSDT', 'ADAUSDT', 'AVAXUSDT'];

interface AnalysisForm {
    watch_pairs: string;
    ai_model: string;
    min_confidence: number;
}

export const Analytics: React.FC = () => {
    const { loadAccuracy } = useAnalyticsActions();
    const { accuracy, loading, error } = useStateAnalytics();

    const [form, setForm] = useState<AnalysisForm>({
        watch_pairs: 'BTCUSDT,ETHUSDT',
        ai_model: 'llama-3.3-70b-versatile',
        min_confidence: 70,
    });

    const [analyzing, setAnalyzing] = useState(false);
    const [results, setResults] = useState<Prediction[]>([]);
    const [snackbar, setSnackbar] = useState<{ open: boolean; message: string; severity: 'success' | 'error' }>({
        open: false,
        message: '',
        severity: 'success',
    });

    useEffect(() => {
        loadAccuracy();

        predictionRequests.fetchSettings().then((settings) => {
            setForm((prev) => ({
                ...prev,
                watch_pairs: settings.watch_pairs,
                ai_model: settings.ai_model,
            }));
        }).catch(() => {});

        const interval = setInterval(() => {
            loadAccuracy();
        }, 60000);

        return () => clearInterval(interval);
    }, []);

    const handleRunAnalysis = async () => {
        try {
            setAnalyzing(true);
            setResults([]);

            await predictionRequests.updateSettings({
                watch_pairs: form.watch_pairs,
                ai_model: form.ai_model,
            });

            await predictionRequests.triggerAnalysis();

            const predictions = await predictionRequests.fetchPredictions({
                min_confidence: form.min_confidence,
                limit: 20,
            });

            setResults(predictions);
            setSnackbar({ open: true, message: `Analysis complete! ${predictions.length} predictions returned.`, severity: 'success' });

            loadAccuracy();
        } catch {
            setSnackbar({
                open: true,
                message: 'Analysis failed. Check your API keys in Settings.',
                severity: 'error',
            });
        } finally {
            setAnalyzing(false);
        }
    };

    const handleSymbolToggle = (symbol: string) => {
        const current = form.watch_pairs.split(',').map((s) => s.trim()).filter(Boolean);
        const idx = current.indexOf(symbol);
        if (idx >= 0) {
            if (current.length === 1) return;
            current.splice(idx, 1);
        } else {
            current.push(symbol);
        }
        setForm({ ...form, watch_pairs: current.join(',') });
    };

    const selectedSymbols = form.watch_pairs.split(',').map((s) => s.trim()).filter(Boolean);

    if (loading && !accuracy) {
        return (
            <Box display="flex" justifyContent="center" alignItems="center" minHeight={400}>
                <CircularProgress color="primary" />
            </Box>
        );
    }

    return (
        <Box sx={{ p: 3 }}>
            <Typography variant="h4" mb={3}>
                Prediction Analytics
            </Typography>

            {error && (
                <Alert severity="error" sx={{ mb: 2 }}>
                    {error}
                </Alert>
            )}

            <Card sx={{ backgroundColor: 'background.paper', mb: 3 }}>
                <CardContent>
                    <Typography variant="h6" mb={2}>
                        Run Analysis
                    </Typography>

                    <Box mb={2}>
                        <Typography variant="body2" fontWeight={600} mb={1}>
                            Trading Pairs
                        </Typography>
                        <Box display="flex" flexWrap="wrap" gap={1}>
                            {SYMBOLS.map((symbol) => (
                                <Button
                                    key={symbol}
                                    variant={selectedSymbols.includes(symbol) ? 'contained' : 'outlined'}
                                    size="small"
                                    onClick={() => handleSymbolToggle(symbol)}
                                    disabled={analyzing}
                                    sx={{
                                        minWidth: 'auto',
                                        fontSize: '0.75rem',
                                        fontWeight: 600,
                                    }}
                                >
                                    {symbol.replace('USDT', '')}
                                </Button>
                            ))}
                        </Box>
                    </Box>

                    <Grid container spacing={2} mb={2}>
                        <Grid size={{ xs: 12, sm: 6 }}>
                            <TextField
                                fullWidth
                                size="small"
                                label="AI Model"
                                select
                                value={form.ai_model}
                                onChange={(e) => setForm({ ...form, ai_model: e.target.value })}
                                disabled={analyzing}
                            >
                                <MenuItem value="llama-3.3-70b-versatile">Llama 3.3 70B</MenuItem>
                                <MenuItem value="llama-3.1-8b-instant">Llama 3.1 8B (fast)</MenuItem>
                                <MenuItem value="mixtral-8x7b-32768">Mixtral 8x7B</MenuItem>
                                <MenuItem value="gemma2-9b-it">Gemma 2 9B</MenuItem>
                            </TextField>
                        </Grid>
                        <Grid size={{ xs: 12, sm: 6 }}>
                            <TextField
                                fullWidth
                                size="small"
                                label="Min Confidence (%)"
                                type="number"
                                value={form.min_confidence}
                                onChange={(e) =>
                                    setForm({ ...form, min_confidence: Math.max(0, Math.min(100, parseInt(e.target.value) || 0)) })
                                }
                                disabled={analyzing}
                                slotProps={{ htmlInput: { min: 0, max: 100 } }}
                            />
                        </Grid>
                    </Grid>

                    <Button
                        variant="contained"
                        fullWidth
                        size="large"
                        startIcon={analyzing ? <CircularProgress size={20} color="inherit" /> : <PlayArrowIcon />}
                        onClick={handleRunAnalysis}
                        disabled={analyzing}
                        sx={{ fontWeight: 700 }}
                    >
                        {analyzing ? 'Analyzing...' : 'Run Analysis'}
                    </Button>
                </CardContent>
            </Card>

            {results.length > 0 && (
                <Box mb={3}>
                    <Typography variant="h6" mb={2}>
                        Analysis Results ({results.length})
                    </Typography>
                    <Grid container spacing={2}>
                        {results.map((prediction) => (
                            <Grid size={{ xs: 12, sm: 6, md: 4 }} key={prediction.id}>
                                <PredictionCard prediction={prediction} />
                            </Grid>
                        ))}
                    </Grid>
                </Box>
            )}

            {accuracy ? (
                <>
                    <Typography variant="h6" mb={2}>
                        Overall Statistics
                    </Typography>
                    <Grid container spacing={2} mb={3}>
                        <Grid size={{ xs: 6, md: 3 }}>
                            <StatsCard
                                label="Total Predictions"
                                value={accuracy.totalPredictions}
                                subtitle="All time"
                            />
                        </Grid>
                        <Grid size={{ xs: 6, md: 3 }}>
                            <StatsCard
                                label="Accuracy"
                                value={`${accuracy.accuracyPercentage.toFixed(1)}%`}
                                color={
                                    accuracy.accuracyPercentage >= 80
                                        ? '#00e676'
                                        : accuracy.accuracyPercentage >= 60
                                          ? '#ffab00'
                                          : '#ff1744'
                                }
                                subtitle={`${accuracy.correct} correct / ${accuracy.incorrect} incorrect`}
                            />
                        </Grid>
                        <Grid size={{ xs: 6, md: 3 }}>
                            <StatsCard
                                label="Correct"
                                value={accuracy.correct}
                                color="#00e676"
                            />
                        </Grid>
                        <Grid size={{ xs: 6, md: 3 }}>
                            <StatsCard
                                label="Pending"
                                value={accuracy.pending}
                                color="#ffab00"
                                subtitle="Awaiting outcome"
                            />
                        </Grid>
                    </Grid>

                    {Object.keys(accuracy.bySymbol).length > 0 && (
                        <AccuracyChart bySymbol={accuracy.bySymbol} />
                    )}
                </>
            ) : (
                <Box display="flex" justifyContent="center" alignItems="center" minHeight={300}>
                    <Typography color="text.secondary">
                        No accuracy data available yet. Run an analysis to get started.
                    </Typography>
                </Box>
            )}

            <Snackbar
                open={snackbar.open}
                autoHideDuration={4000}
                onClose={() => setSnackbar({ ...snackbar, open: false })}
            >
                <Alert severity={snackbar.severity} onClose={() => setSnackbar({ ...snackbar, open: false })}>
                    {snackbar.message}
                </Alert>
            </Snackbar>
        </Box>
    );
};
