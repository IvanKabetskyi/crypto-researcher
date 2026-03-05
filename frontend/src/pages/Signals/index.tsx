import { useState } from 'react';
import { Box, Typography, Grid, CircularProgress, Alert, Snackbar } from '@mui/material';
import { useSignalsActions } from './hooks/useSignalsActions';
import { useStateSignals } from './hooks/useStateSignals';
import { PredictionCard } from './components/PredictionCard';
import { SignalForm } from './components/SignalForm';

export const Signals = () => {
    const { runAnalysis } = useSignalsActions();
    const { predictions, analyzing, error } = useStateSignals();
    const [directionFilter, setDirectionFilter] = useState('all');
    const [snackbar, setSnackbar] = useState<{
        open: boolean;
        message: string;
        severity: 'success' | 'error';
    }>({
        open: false,
        message: '',
        severity: 'success',
    });

    const handleRunAnalysis = async (params: {
        pairs: string[];
        timeframe: string;
        min_confidence: number;
    }) => {
        try {
            await runAnalysis(params).unwrap();
            setSnackbar({
                open: true,
                message: 'Analysis complete!',
                severity: 'success',
            });
        } catch {
            setSnackbar({
                open: true,
                message: 'Analysis failed. Check your API keys.',
                severity: 'error',
            });
        }
    };

    const filteredPredictions =
        directionFilter === 'all'
            ? predictions
            : predictions.filter((p) => p.direction === directionFilter);

    return (
        <Box sx={{ p: 3 }}>
            <Typography variant="h4" mb={3}>
                Signals
            </Typography>

            <SignalForm
                onSubmit={handleRunAnalysis}
                analyzing={analyzing}
                directionFilter={directionFilter}
                onDirectionFilterChange={setDirectionFilter}
            />

            {error && (
                <Alert severity="error" sx={{ mb: 2 }}>
                    {error}
                </Alert>
            )}

            {analyzing && predictions.length === 0 ? (
                <Box display="flex" justifyContent="center" alignItems="center" minHeight={300}>
                    <CircularProgress color="primary" />
                </Box>
            ) : filteredPredictions.length === 0 ? (
                <Box display="flex" justifyContent="center" alignItems="center" minHeight={300}>
                    <Typography color="text.secondary" textAlign="center">
                        No predictions yet. Select pairs and timeframe, then click "Run Analysis".
                    </Typography>
                </Box>
            ) : (
                <Grid container spacing={2}>
                    {filteredPredictions.map((prediction) => (
                        <Grid size={{ xs: 12, sm: 6, md: 4 }} key={prediction.id}>
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
                <Alert
                    severity={snackbar.severity}
                    onClose={() => setSnackbar({ ...snackbar, open: false })}
                >
                    {snackbar.message}
                </Alert>
            </Snackbar>
        </Box>
    );
};
