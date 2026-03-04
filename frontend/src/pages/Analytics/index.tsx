import React, { useEffect } from 'react';
import { Box, Typography, Grid, CircularProgress, Alert } from '@mui/material';
import { useAnalyticsActions } from './hooks/useAnalyticsActions';
import { useStateAnalytics } from './hooks/useStateAnalytics';
import { StatsCard } from './components/StatsCard';
import { AccuracyChart } from './components/AccuracyChart';

export const Analytics: React.FC = () => {
    const { loadAccuracy } = useAnalyticsActions();
    const { accuracy, loading, error } = useStateAnalytics();

    useEffect(() => {
        loadAccuracy();

        const interval = setInterval(() => {
            loadAccuracy();
        }, 60000);

        return () => clearInterval(interval);
    }, []);

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

            {accuracy ? (
                <>
                    <Grid container spacing={2} mb={3}>
                        <Grid item xs={6} md={3}>
                            <StatsCard
                                label="Total Predictions"
                                value={accuracy.totalPredictions}
                                subtitle="All time"
                            />
                        </Grid>
                        <Grid item xs={6} md={3}>
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
                        <Grid item xs={6} md={3}>
                            <StatsCard
                                label="Correct"
                                value={accuracy.correct}
                                color="#00e676"
                            />
                        </Grid>
                        <Grid item xs={6} md={3}>
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
                        No accuracy data available yet. Predictions need time to resolve.
                    </Typography>
                </Box>
            )}
        </Box>
    );
};
