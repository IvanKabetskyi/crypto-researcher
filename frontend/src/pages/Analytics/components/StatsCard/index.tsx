import React from 'react';
import { Card, CardContent, Typography, Box } from '@mui/material';

interface StatsCardProps {
    label: string;
    value: string | number;
    color?: string;
    subtitle?: string;
}

export const StatsCard: React.FC<StatsCardProps> = ({ label, value, color, subtitle }) => {
    return (
        <Card sx={{ backgroundColor: 'background.paper', height: '100%' }}>
            <CardContent>
                <Typography variant="body2" color="text.secondary" gutterBottom>
                    {label}
                </Typography>
                <Typography
                    variant="h4"
                    fontWeight={700}
                    sx={{ color: color || 'text.primary' }}
                >
                    {value}
                </Typography>
                {subtitle && (
                    <Box mt={0.5}>
                        <Typography variant="caption" color="text.secondary">
                            {subtitle}
                        </Typography>
                    </Box>
                )}
            </CardContent>
        </Card>
    );
};
