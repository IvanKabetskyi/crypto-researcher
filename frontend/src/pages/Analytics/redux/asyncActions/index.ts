import { createAsyncThunk } from '@reduxjs/toolkit';
import { predictionRequests } from 'core/Gateways/Prediction/requests';
import { AccuracyStats } from 'core/Entities/Accuracy/types';

export const fetchAccuracy = createAsyncThunk<AccuracyStats>(
    'analytics/fetchAccuracy',
    async () => {
        return await predictionRequests.fetchAccuracy();
    },
);
