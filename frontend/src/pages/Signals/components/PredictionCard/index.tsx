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

const getRiskColor = (decision?: string): string => {
    switch (decision) {
        case 'APPROVE':
            return '#00e676';
        case 'REDUCE_SIZE':
            return '#ffab00';
        case 'REJECT':
            return '#ff1744';
        default:
            return '#9e9e9e';
    }
};

const getExecutionColor = (action?: string): string => {
    switch (action) {
        case 'ENTER_NOW':
            return '#00e676';
        case 'SCALE_IN':
            return '#29b6f6';
        case 'WAIT_CONFIRMATION':
            return '#ffab00';
        case 'REDUCED_SIZE':
            return '#ffa726';
        case 'SKIP_TRADE':
            return '#ff1744';
        default:
            return '#9e9e9e';
    }
};

const getBiasColor = (bias?: string): string => {
    switch (bias) {
        case 'bullish':
            return '#00e676';
        case 'bearish':
            return '#ff1744';
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

                {/* Pipeline tags row */}
                <Box display="flex" flexWrap="wrap" gap={0.5} mb={1.5}>
                    {prediction.timeframe && (
                        <Chip label={prediction.timeframe} size="small" variant="outlined" />
                    )}
                    {prediction.marketBias && (
                        <Chip
                            label={prediction.marketBias.toUpperCase()}
                            size="small"
                            sx={{
                                backgroundColor: `${getBiasColor(prediction.marketBias)}15`,
                                color: getBiasColor(prediction.marketBias),
                                fontWeight: 600,
                                fontSize: '0.7rem',
                            }}
                        />
                    )}
                    {prediction.setupType && (
                        <Chip
                            label={prediction.setupType.replace('_', ' ')}
                            size="small"
                            variant="outlined"
                            sx={{ fontSize: '0.7rem' }}
                        />
                    )}
                    {prediction.riskDecision && (
                        <Chip
                            label={`Risk: ${prediction.riskDecision}`}
                            size="small"
                            sx={{
                                backgroundColor: `${getRiskColor(prediction.riskDecision)}15`,
                                color: getRiskColor(prediction.riskDecision),
                                fontWeight: 600,
                                fontSize: '0.7rem',
                            }}
                        />
                    )}
                    {prediction.executionAction && (
                        <Chip
                            label={prediction.executionAction.replace('_', ' ')}
                            size="small"
                            sx={{
                                backgroundColor: `${getExecutionColor(prediction.executionAction)}15`,
                                color: getExecutionColor(prediction.executionAction),
                                fontWeight: 600,
                                fontSize: '0.7rem',
                            }}
                        />
                    )}
                </Box>

                {/* Price levels */}
                <Box display="flex" gap={2} mb={1.5} flexWrap="wrap">
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
                    {prediction.secondaryTarget && (
                        <Box>
                            <Typography variant="caption" color="text.secondary">
                                Target 2
                            </Typography>
                            <Typography variant="body2" fontWeight={600} sx={{ color: '#29b6f6' }}>
                                ${formatPrice(prediction.secondaryTarget)}
                            </Typography>
                        </Box>
                    )}
                    <Box>
                        <Typography variant="caption" color="text.secondary">
                            Stop Loss
                        </Typography>
                        <Typography variant="body2" fontWeight={600} color="secondary">
                            ${formatPrice(prediction.stopLoss)}
                        </Typography>
                    </Box>
                    {prediction.invalidation && (
                        <Box>
                            <Typography variant="caption" color="text.secondary">
                                Invalidation
                            </Typography>
                            <Typography variant="body2" fontWeight={600} sx={{ color: '#ff5252' }}>
                                ${formatPrice(prediction.invalidation)}
                            </Typography>
                        </Box>
                    )}
                </Box>

                {/* Risk/Reward and Position Size row */}
                {(prediction.riskRewardRatio || prediction.positionSizePct) && (
                    <Box display="flex" gap={2} mb={1.5}>
                        {prediction.riskRewardRatio != null && (
                            <Box>
                                <Typography variant="caption" color="text.secondary">
                                    R:R Ratio
                                </Typography>
                                <Typography variant="body2" fontWeight={600}>
                                    {prediction.riskRewardRatio.toFixed(1)}:1
                                </Typography>
                            </Box>
                        )}
                        {prediction.positionSizePct != null && (
                            <Box>
                                <Typography variant="caption" color="text.secondary">
                                    Position Size
                                </Typography>
                                <Typography variant="body2" fontWeight={600}>
                                    {prediction.positionSizePct.toFixed(0)}%
                                </Typography>
                            </Box>
                        )}
                        {prediction.reviewConfidence != null && (
                            <Box>
                                <Typography variant="caption" color="text.secondary">
                                    Review Score
                                </Typography>
                                <Typography
                                    variant="body2"
                                    fontWeight={600}
                                    sx={{ color: prediction.reviewAgreed ? '#00e676' : '#ffab00' }}
                                >
                                    {prediction.reviewConfidence.toFixed(0)}%
                                    {prediction.reviewAgreed === false && ' (cautious)'}
                                </Typography>
                            </Box>
                        )}
                    </Box>
                )}

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
