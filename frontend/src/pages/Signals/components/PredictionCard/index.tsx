import React, { useState } from 'react';
import { Card, CardContent, Typography, Box, Chip, Divider, Collapse, IconButton, List, ListItem, ListItemText, Alert } from '@mui/material';
import TrendingUpIcon from '@mui/icons-material/TrendingUp';
import TrendingDownIcon from '@mui/icons-material/TrendingDown';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import VerifiedIcon from '@mui/icons-material/Verified';
import { Prediction } from 'types/prediction';
import { ConfidenceBadge } from '../ConfidenceBadge';
import { formatPrice } from 'utils/formatting';
import {
    getStatusConfig,
    biasText,
    momentumText,
    volumeText,
    trendText,
    derivativesText,
} from '../../utils/statusHelpers';

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
    const status = getStatusConfig(prediction.predictionStatus);
    const isRejected = prediction.predictionStatus === 'REJECTED';

    const marketSummaryLines: string[] = [];
    if (prediction.marketBias && biasText[prediction.marketBias]) {
        marketSummaryLines.push(biasText[prediction.marketBias]);
    }
    if (prediction.trendStrength && trendText[prediction.trendStrength]) {
        marketSummaryLines.push(trendText[prediction.trendStrength]);
    }
    if (prediction.momentum && momentumText[prediction.momentum]) {
        marketSummaryLines.push(momentumText[prediction.momentum]);
    }
    if (prediction.volumeProfile && volumeText[prediction.volumeProfile]) {
        marketSummaryLines.push(volumeText[prediction.volumeProfile]);
    }
    if (prediction.derivativesSentiment && derivativesText[prediction.derivativesSentiment]) {
        marketSummaryLines.push(derivativesText[prediction.derivativesSentiment]);
    }

    return (
        <Card
            sx={{
                backgroundColor: 'background.paper',
                transition: 'transform 0.2s, box-shadow 0.2s',
                cursor: 'pointer',
                opacity: isRejected ? 0.7 : 1,
                '&:hover': {
                    transform: 'translateY(-2px)',
                    boxShadow: `0 4px 20px rgba(${isLong ? '0, 230, 118' : '255, 23, 68'}, 0.15)`,
                },
            }}
            onClick={() => setExpanded(!expanded)}
        >
            <CardContent>
                {/* Section 1: Header — Symbol + Direction + Status Badge */}
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
                        <Chip
                            label={status.label}
                            size="small"
                            sx={{
                                backgroundColor: `${status.color}20`,
                                color: status.color,
                                fontWeight: 700,
                                fontSize: '0.7rem',
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
                    {prediction.setupType && (
                        <Chip
                            label={prediction.setupType.replace(/_/g, ' ')}
                            size="small"
                            variant="outlined"
                            sx={{ fontSize: '0.7rem' }}
                        />
                    )}
                </Box>

                {/* Section 2: Market Summary (plain English) */}
                {marketSummaryLines.length > 0 && (
                    <Box
                        sx={{
                            mb: 1.5,
                            p: 1.5,
                            borderRadius: 1,
                            backgroundColor: 'rgba(255,255,255,0.02)',
                            border: '1px solid rgba(255,255,255,0.06)',
                        }}
                    >
                        <Typography variant="caption" fontWeight={700} color="text.secondary" sx={{ mb: 0.5, display: 'block' }}>
                            Market Context
                        </Typography>
                        {marketSummaryLines.map((line, i) => (
                            <Typography key={i} variant="body2" color="text.secondary" sx={{ lineHeight: 1.6 }}>
                                {line}
                            </Typography>
                        ))}
                    </Box>
                )}

                {/* Section 3: Key Levels */}
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
                            Target 1
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

                {/* Section 4: Why This Status */}
                {isRejected && (
                    <Alert severity="error" sx={{ mb: 1.5, '& .MuiAlert-message': { width: '100%' } }}>
                        <Typography variant="body2" fontWeight={600}>
                            This trade was rejected.
                        </Typography>
                        {prediction.reviewNotes && prediction.reviewNotes.length > 0 && (
                            <Typography variant="body2" sx={{ mt: 0.5 }}>
                                {prediction.reviewNotes[0]}
                            </Typography>
                        )}
                    </Alert>
                )}

                {prediction.reviewVerdict && (
                    <Box
                        sx={{
                            mb: 1.5,
                            p: 1.5,
                            borderRadius: 1,
                            backgroundColor: 'rgba(255,255,255,0.03)',
                            border: `1px solid ${status.color}30`,
                        }}
                    >
                        <Box display="flex" alignItems="center" gap={1} mb={1}>
                            <VerifiedIcon sx={{ fontSize: 16, color: status.color }} />
                            <Typography variant="caption" fontWeight={700} sx={{ color: status.color }}>
                                Review AI
                            </Typography>
                            {prediction.reviewConfidence != null && (
                                <Typography variant="caption" fontWeight={600} sx={{ ml: 'auto', color: 'text.secondary' }}>
                                    {prediction.reviewConfidence.toFixed(0)}%
                                </Typography>
                            )}
                        </Box>

                        {prediction.reviewIssues && prediction.reviewIssues.length > 0 && (
                            <List dense disablePadding sx={{ mb: 0.5 }}>
                                {prediction.reviewIssues.map((issue, i) => (
                                    <ListItem key={i} disablePadding sx={{ py: 0 }}>
                                        <ListItemText
                                            primary={`\u2022 ${issue}`}
                                            primaryTypographyProps={{
                                                variant: 'caption',
                                                color: '#ffa726',
                                                sx: { lineHeight: 1.4 },
                                            }}
                                        />
                                    </ListItem>
                                ))}
                            </List>
                        )}

                        {prediction.reviewNotes && prediction.reviewNotes.length > 0 && !isRejected && (
                            <List dense disablePadding>
                                {prediction.reviewNotes.map((note, i) => (
                                    <ListItem key={i} disablePadding sx={{ py: 0 }}>
                                        <ListItemText
                                            primary={note}
                                            primaryTypographyProps={{
                                                variant: 'caption',
                                                color: 'text.secondary',
                                                sx: { lineHeight: 1.4 },
                                            }}
                                        />
                                    </ListItem>
                                ))}
                            </List>
                        )}
                    </Box>
                )}

                {/* Section 5: Risk/Review details */}
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
                    </Box>
                )}

                <Divider sx={{ my: 1, borderColor: 'rgba(255,255,255,0.06)' }} />

                {/* Section 6: Full reasoning (expandable) */}
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
