import { useEffect, useState, useRef } from 'react';
import { Box, Typography, Grid, CircularProgress, Alert, Snackbar, LinearProgress } from '@mui/material';
import { useSignalsActions } from './hooks/useSignalsActions';
import { useStateSignals } from './hooks/useStateSignals';
import { PredictionCard } from './components/PredictionCard';
import { SignalForm } from './components/SignalForm';

const PIPELINE_STAGES = [
    'Fetching market data...',
    'Stage 1: Analyzing market conditions...',
    'Stage 2: Classifying setups (Opus)...',
    'Stage 3: Assessing risk...',
    'Stage 4: Optimizing strategy...',
    'Stage 5: Final review...',
    'Saving results...',
];

export const Signals = () => {
    const { runAnalysis, fetchConfig } = useSignalsActions();
    const { predictions, analyzing, error, availablePairs, availableTimeframes, configLoaded } = useStateSignals();
    const [elapsed, setElapsed] = useState(0);
    const [stageIndex, setStageIndex] = useState(0);
    const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

    useEffect(() => {
        if (analyzing) {
            setElapsed(0);
            setStageIndex(0);
            timerRef.current = setInterval(() => {
                setElapsed((prev) => {
                    const next = prev + 1;
                    // Estimate stage based on elapsed time
                    if (next >= 90) setStageIndex(6);
                    else if (next >= 70) setStageIndex(5);
                    else if (next >= 50) setStageIndex(4);
                    else if (next >= 35) setStageIndex(3);
                    else if (next >= 15) setStageIndex(2);
                    else if (next >= 5) setStageIndex(1);
                    return next;
                });
            }, 1000);
        } else {
            if (timerRef.current) {
                clearInterval(timerRef.current);
                timerRef.current = null;
            }
        }
        return () => {
            if (timerRef.current) clearInterval(timerRef.current);
        };
    }, [analyzing]);

    useEffect(() => {
        if (!configLoaded) {
            fetchConfig();
        }
    }, [configLoaded]);
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
        bet_value: number;
    }) => {
        try {
            await runAnalysis(params).unwrap();
            setSnackbar({
                open: true,
                message: 'Analysis complete!',
                severity: 'success',
            });
        } catch (err: unknown) {
            const msg = (err as { message?: string })?.message
                || String(err)
                || 'Analysis failed';
            setSnackbar({
                open: true,
                message: msg,
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
                availablePairs={availablePairs}
                availableTimeframes={availableTimeframes}
            />

            {error && (
                <Alert severity="error" sx={{ mb: 2 }}>
                    {error}
                </Alert>
            )}

            {analyzing && predictions.length === 0 ? (
                <Box display="flex" flexDirection="column" justifyContent="center" alignItems="center" minHeight={300} gap={2}>
                    <CircularProgress color="primary" />
                    <Typography variant="body2" color="text.secondary" fontWeight={600}>
                        {PIPELINE_STAGES[stageIndex]}
                    </Typography>
                    <Box sx={{ width: '60%', maxWidth: 400 }}>
                        <LinearProgress
                            variant="determinate"
                            value={Math.min((elapsed / 120) * 100, 95)}
                            sx={{ height: 6, borderRadius: 3 }}
                        />
                    </Box>
                    <Typography variant="caption" color="text.secondary">
                        {elapsed}s elapsed
                    </Typography>
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
