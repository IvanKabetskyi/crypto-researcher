import React from 'react';
import { Box, Card, CardContent, Typography } from '@mui/material';
import TrendingUpIcon from '@mui/icons-material/TrendingUp';
import TrendingDownIcon from '@mui/icons-material/TrendingDown';
import { MarketData } from 'core/Entities/Market/types';

interface MarketOverviewProps {
    market: MarketData[];
}

const formatVolume = (volume: number): string => {
    if (volume >= 1_000_000_000) return `$${(volume / 1_000_000_000).toFixed(2)}B`;
    if (volume >= 1_000_000) return `$${(volume / 1_000_000).toFixed(2)}M`;
    if (volume >= 1_000) return `$${(volume / 1_000).toFixed(2)}K`;
    return `$${volume.toFixed(2)}`;
};

export const MarketOverview: React.FC<MarketOverviewProps> = ({ market }) => {
    return (
        <Box
            sx={{
                display: 'flex',
                gap: 2,
                overflowX: 'auto',
                pb: 1,
                '&::-webkit-scrollbar': { height: 4 },
                '&::-webkit-scrollbar-thumb': {
                    backgroundColor: 'rgba(255,255,255,0.1)',
                    borderRadius: 2,
                },
            }}
        >
            {market.map((item) => {
                const isPositive = item.change24h >= 0;
                const changeColor = isPositive ? '#00e676' : '#ff1744';

                return (
                    <Card
                        key={item.symbol}
                        sx={{
                            minWidth: 180,
                            backgroundColor: 'background.paper',
                            flexShrink: 0,
                        }}
                    >
                        <CardContent sx={{ py: 1.5, '&:last-child': { pb: 1.5 } }}>
                            <Typography variant="body2" fontWeight={700}>
                                {item.symbol}
                            </Typography>
                            <Typography variant="h6" fontWeight={700}>
                                $
                                {item.price >= 1
                                    ? item.price.toLocaleString('en-US', {
                                          maximumFractionDigits: 2,
                                      })
                                    : item.price.toFixed(6)}
                            </Typography>
                            <Box display="flex" alignItems="center" gap={0.5}>
                                {isPositive ? (
                                    <TrendingUpIcon sx={{ fontSize: 16, color: changeColor }} />
                                ) : (
                                    <TrendingDownIcon sx={{ fontSize: 16, color: changeColor }} />
                                )}
                                <Typography
                                    variant="body2"
                                    sx={{ color: changeColor, fontWeight: 600 }}
                                >
                                    {isPositive ? '+' : ''}
                                    {item.change24h.toFixed(2)}%
                                </Typography>
                            </Box>
                            <Typography variant="caption" color="text.secondary">
                                Vol: {formatVolume(item.volume24h)}
                            </Typography>
                        </CardContent>
                    </Card>
                );
            })}
        </Box>
    );
};
