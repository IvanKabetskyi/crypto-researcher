import React, { useState } from 'react';
import { Card, CardContent, Typography, Box, Chip, Divider, Collapse, IconButton } from '@mui/material';
import TrendingUpIcon from '@mui/icons-material/TrendingUp';
import TrendingDownIcon from '@mui/icons-material/TrendingDown';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import { Prediction } from 'types/prediction';
import { ConfidenceBadge } from '../ConfidenceBadge';
import { formatPrice } from 'utils/formatting';

interface PredictionCardProps {
    prediction: Prediction;
}

const getOutcomeColor = (outcome: Prediction['outcome']): string => {
    switch (outcome) {
        case 'correct':
            return '#00e676';
        case 'incorrect':
            return '#ff1744';
        case 'pending':
            return '#ffab00';
        default:
            return '#9e9e9e';
    }
};

export const PredictionCard: React.FC<PredictionCardProps> = ({ prediction }) => {
    const [expanded, setExpanded] = useState(false);
    const isLong = prediction.direction === 'long';
    const directionColor = isLong ? '#00e676' : '#ff1744';

    return (
        <Card
            sx={{
                backgroundColor: 'background.paper',
                transition: 'transform 0.2s, box-shadow 0.2s',
                cursor: 'pointer',
                '&:hover': {
                    transform: 'translateY(-2px)',
                    boxShadow: `0 4px 20px rgba(${isLong ? '0, 230, 118' : '255, 23, 68'}, 0.15)`,
                },
            }}
            onClick={() => setExpanded(!expanded)}
        >
            <CardContent>
                <Box display="flex" justifyContent="space-between" alignItems="center" mb={1.5}>
                    <Box display="flex" alignItems="center" gap={1}>
                        <Typography variant="h6" fontWeight={700}>
                            {prediction.symbol}
                        </Typography>
                        <Chip
                            icon={isLong ? <TrendingUpIcon /> : <TrendingDownIcon />}
                            label={prediction.direction.toUpperCase()}
                            size="small"
                            sx={{
                                backgroundColor: `${directionColor}20`,
                                color: directionColor,
                                fontWeight: 700,
                            }}
                        />
                    </Box>
                    <Box display="flex" alignItems="center" gap={0.5}>
                        <ConfidenceBadge confidence={prediction.confidence} />
                        <IconButton
                            size="small"
                            sx={{
                                transform: expanded ? 'rotate(180deg)' : 'rotate(0deg)',
                                transition: 'transform 0.3s',
                                color: 'text.secondary',
                            }}
                            onClick={(e) => {
                                e.stopPropagation();
                                setExpanded(!expanded);
                            }}
                        >
                            <ExpandMoreIcon />
                        </IconButton>
                    </Box>
                </Box>

                {prediction.timeframe && (
                    <Box mb={1}>
                        <Chip label={prediction.timeframe} size="small" variant="outlined" />
                    </Box>
                )}

                <Box display="flex" gap={2} mb={1.5}>
                    <Box>
                        <Typography variant="caption" color="text.secondary">
                            Entry
                        </Typography>
                        <Typography variant="body2" fontWeight={600}>
                            ${formatPrice(prediction.entryPrice)}
                        </Typography>
                    </Box>
                    <Box>
                        <Typography variant="caption" color="text.secondary">
                            Target
                        </Typography>
                        <Typography variant="body2" fontWeight={600} color="primary">
                            ${formatPrice(prediction.targetPrice)}
                        </Typography>
                    </Box>
                    <Box>
                        <Typography variant="caption" color="text.secondary">
                            Stop Loss
                        </Typography>
                        <Typography variant="body2" fontWeight={600} color="secondary">
                            ${formatPrice(prediction.stopLoss)}
                        </Typography>
                    </Box>
                </Box>

                <Divider sx={{ my: 1, borderColor: 'rgba(255,255,255,0.06)' }} />

                <Collapse in={!expanded} timeout="auto">
                    <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{
                            overflow: 'hidden',
                            textOverflow: 'ellipsis',
                            display: '-webkit-box',
                            WebkitLineClamp: 2,
                            WebkitBoxOrient: 'vertical',
                            mb: 1,
                        }}
                    >
                        {prediction.reasoning}
                    </Typography>
                </Collapse>

                <Collapse in={expanded} timeout="auto">
                    <Typography
                        variant="body2"
                        color="text.secondary"
                        sx={{ mb: 1, whiteSpace: 'pre-wrap' }}
                    >
                        {prediction.reasoning}
                    </Typography>
                </Collapse>

                <Box display="flex" justifyContent="space-between" alignItems="center">
                    <Typography variant="caption" color="text.secondary">
                        {new Date(prediction.createdAt).toLocaleString()}
                    </Typography>
                    {prediction.outcome && (
                        <Chip
                            label={prediction.outcome.toUpperCase()}
                            size="small"
                            sx={{
                                backgroundColor: `${getOutcomeColor(prediction.outcome)}20`,
                                color: getOutcomeColor(prediction.outcome),
                                fontWeight: 600,
                                fontSize: '0.7rem',
                            }}
                        />
                    )}
                </Box>
            </CardContent>
        </Card>
    );
};
