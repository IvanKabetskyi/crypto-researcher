import React from 'react';
import { Chip } from '@mui/material';

interface ConfidenceBadgeProps {
    confidence: number;
}

const getColor = (confidence: number): 'success' | 'warning' | 'error' | 'default' => {
    if (confidence >= 85) return 'success';
    if (confidence >= 80) return 'warning';
    return 'default';
};

export const ConfidenceBadge: React.FC<ConfidenceBadgeProps> = ({ confidence }) => {
    return (
        <Chip
            label={`${confidence.toFixed(1)}%`}
            color={getColor(confidence)}
            size="small"
            sx={{ fontWeight: 700, fontSize: '0.85rem' }}
        />
    );
};
