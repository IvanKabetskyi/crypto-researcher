import { useEffect, useState, useRef, useCallback } from 'react';
import { Box, Typography, Grid, CircularProgress, Alert, Snackbar, LinearProgress, Chip } from '@mui/material';
import { useSignalsActions } from './hooks/useSignalsActions';
import { useStateSignals } from './hooks/useStateSignals';
import { PredictionCard } from './components/PredictionCard';
import { SignalForm } from './components/SignalForm';

export const Signals = () => {
    const { runAnalysis, fetchConfig } = useSignalsActions();
    const { predictions, analyzing, error, availablePairs, availableTimeframes, configLoaded } = useStateSignals();
    const [elapsed, setElapsed] = useState(0);
    const [currentStage, setCurrentStage] = useState('Starting...');
    const [completedStages, setCompletedStages] = useState<string[]>([]);
    const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);
    const lastStageRef = useRef('');

    useEffect(() => {
        if (analyzing) {
            setElapsed(0);
            setCurrentStage('Starting...');
            setCompletedStages([]);
            lastStageRef.current = '';
            timerRef.current = setInterval(() => {
                setElapsed((prev) => prev + 1);
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

    const handleStageUpdate = useCallback((stage: string) => {
        if (stage !== lastStageRef.current) {
            if (lastStageRef.current) {
                setCompletedStages((prev) => [...prev, lastStageRef.current]);
            }
            lastStageRef.current = stage;
            setCurrentStage(stage);
        }
    }, []);

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
            await runAnalysis({ ...params, onStage: handleStageUpdate }).unwrap();
            setSnackbar({ open: true, message: 'Analysis complete!', severity: 'success' });
        } catch (err: unknown) {
            const msg = (err as { message?: string })?.message || String(err) || 'Analysis failed';
            setSnackbar({ open: true, message: msg, severity: 'error' });
        }
    };

    const filteredPredictions =
        directionFilter === 'all'
            ? predictions
            : predictions.filter((p) => p.direction === directionFilter);

    // Estimate progress from stage number
    const stageMatch = currentStage.match(/Stage (\d)\/5/);
    const stageNum = stageMatch ? parseInt(stageMatch[1]) : 0;
    const progress = analyzing
        ? Math.min(((stageNum > 0 ? (stageNum - 1) * 20 : 0) + Math.min(elapsed % 60, 18)), 95)
        : 0;

    return (
        <Box sx={{ p: 3 }}>
            <Typography variant="h5" mb={2} fontWeight={700}>
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

            {analyzing ? (
                <Box sx={{ mt: 2 }}>
                    {/* Progress bar */}
                    <Box sx={{ mb: 2 }}>
                        <Box display="flex" justifyContent="space-between" alignItems="center" mb={0.5}>
                            <Typography variant="body2" fontWeight={600} color="primary">
                                {currentStage}
                            </Typography>
                            <Typography variant="caption" color="text.secondary">
                                {elapsed}s
                            </Typography>
                        </Box>
                        <LinearProgress
                            variant="determinate"
                            value={progress}
                            sx={{ height: 4, borderRadius: 2 }}
                        />
                    </Box>

                    {/* Completed stages */}
                    {completedStages.length > 0 && (
                        <Box display="flex" gap={0.5} flexWrap="wrap" mb={2}>
                            {completedStages.map((s, i) => (
                                <Chip
                                    key={i}
                                    label={s}
                                    size="small"
                                    color="success"
                                    variant="outlined"
                                    sx={{ fontSize: '0.7rem' }}
                                />
                            ))}
                        </Box>
                    )}

                    <Box display="flex" justifyContent="center" mt={4}>
                        <CircularProgress size={32} />
                    </Box>
                </Box>
            ) : filteredPredictions.length === 0 ? (
                <Box display="flex" justifyContent="center" alignItems="center" minHeight={200}>
                    <Typography color="text.secondary" textAlign="center">
                        No predictions yet. Select pairs and timeframe, then click "Analyze".
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
