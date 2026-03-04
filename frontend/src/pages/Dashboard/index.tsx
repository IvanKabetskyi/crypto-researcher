import { useEffect, useState } from 'react';
import { Box, Typography, Grid, CircularProgress, Alert, Button, Snackbar } from '@mui/material';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import { useDashboardActions } from './hooks/useDashboardActions';
import { useStateDashboard } from './hooks/useStateDashboard';
import { PredictionCard } from './components/PredictionCard';
import { MarketOverview } from './components/MarketOverview';
import { FilterPanel } from 'widgets/FilterPanel';
import { predictionRequests } from 'core/Gateways/Prediction/requests';

const REFRESH_INTERVAL = 30000;

export const Dashboard = () => {
    const { loadPredictions, loadMarket } = useDashboardActions();
    const { predictions, market, loading, error, lastUpdated } = useStateDashboard();
    const [analyzing, setAnalyzing] = useState(false);
    const [snackbar, setSnackbar] = useState<{ open: boolean; message: string; severity: 'success' | 'error' }>({
        open: false,
        message: '',
        severity: 'success',
    });

    useEffect(() => {
        loadPredictions();
        loadMarket();

        const interval = setInterval(() => {
            loadPredictions();
            loadMarket();
        }, REFRESH_INTERVAL);

        return () => clearInterval(interval);
    }, []);

    const handleRunAnalysis = async () => {
        try {
            setAnalyzing(true);
            await predictionRequests.triggerAnalysis();
            setSnackbar({ open: true, message: 'Analysis complete! Refreshing data...', severity: 'success' });
            await loadPredictions();
            await loadMarket();
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

    return (
        <Box sx={{ p: 3 }}>
            <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
                <Typography variant="h4">Live Predictions</Typography>
                <Box display="flex" alignItems="center" gap={2}>
                    {lastUpdated && (
                        <Typography variant="caption" color="text.secondary">
                            Last updated: {new Date(lastUpdated).toLocaleTimeString()}
                        </Typography>
                    )}
                    <Button
                        variant="contained"
                        startIcon={analyzing ? <CircularProgress size={18} color="inherit" /> : <PlayArrowIcon />}
                        onClick={handleRunAnalysis}
                        disabled={analyzing}
                        sx={{ fontWeight: 700 }}
                    >
                        {analyzing ? 'Analyzing...' : 'Run Analysis'}
                    </Button>
                </Box>
            </Box>

            <Box mb={3}>
                <Typography variant="h6" mb={1.5}>
                    Market Overview
                </Typography>
                <MarketOverview market={market} />
            </Box>

            <Box mb={3}>
                <FilterPanel />
            </Box>

            {error && (
                <Alert severity="error" sx={{ mb: 2 }}>
                    {error}
                </Alert>
            )}

            {loading && predictions.length === 0 ? (
                <Box display="flex" justifyContent="center" alignItems="center" minHeight={300}>
                    <CircularProgress color="primary" />
                </Box>
            ) : predictions.length === 0 ? (
                <Box
                    display="flex"
                    flexDirection="column"
                    justifyContent="center"
                    alignItems="center"
                    minHeight={300}
                    gap={2}
                >
                    <Typography color="text.secondary" textAlign="center">
                        No predictions yet. Configure your API keys in Settings, then click "Run Analysis".
                    </Typography>
                    <Button variant="outlined" href="/settings">
                        Go to Settings
                    </Button>
                </Box>
            ) : (
                <Grid container spacing={2}>
                    {predictions.map((prediction) => (
                        <Grid item xs={12} sm={6} md={4} key={prediction.id}>
                            <PredictionCard prediction={prediction} />
                        </Grid>
                    ))}
                </Grid>
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
