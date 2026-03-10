export const statusConfig: Record<string, { label: string; color: string; description: string }> = {
    APPROVED: { label: 'Approved', color: '#00e676', description: 'Trade idea is acceptable' },
    ACCEPT_WITH_CAUTION: { label: 'Caution', color: '#ffab00', description: 'Valid direction, moderate risks' },
    WAIT_CONFIRMATION: { label: 'Wait for Confirmation', color: '#2196f3', description: 'Do not enter yet' },
    REDUCED_SIZE: { label: 'Reduced Size', color: '#ffa726', description: 'Enter smaller than usual' },
    DOWNGRADED: { label: 'Downgraded', color: '#9c27b0', description: 'Setup got weaker' },
    REJECTED: { label: 'Rejected', color: '#ff1744', description: 'Do not take this setup' },
};

export const biasText: Record<string, string> = {
    bullish: 'Market shows upward pressure',
    bearish: 'Market shows downward pressure',
    neutral: 'Market is neutral',
};

export const momentumText: Record<string, string> = {
    accelerating: 'Momentum is increasing',
    decelerating: 'Momentum is slowing down',
    neutral: 'Momentum is stable',
    steady: 'Momentum is stable',
    exhausted: 'Momentum is exhausted',
};

export const volumeText: Record<string, string> = {
    confirming: 'Volume supports the current move',
    weak: 'Volume support is weak',
    divergent: 'Volume does not fully confirm the move',
    diverging: 'Volume diverges from price action',
    spike: 'Unusual volume spike detected',
};

export const trendText: Record<string, string> = {
    strong: 'Trend is strong',
    moderate: 'Trend is moderate',
    weak: 'Trend is weak',
};

export const derivativesText: Record<string, string> = {
    bullish: 'Derivatives lean bullish',
    bearish: 'Derivatives lean bearish',
    neutral: 'Derivatives are neutral',
    squeeze_risk: 'Squeeze risk detected in derivatives',
};

export const timeframeDescription: Record<string, string> = {
    '5min': 'Scalping — ultra-short trades lasting 5-30 minutes',
    '30min': 'Intraday — short-term trades lasting 1-4 hours',
    '1h': 'Intraday swing — trades lasting 4-12 hours',
    '6h': 'Swing — trades lasting 1-3 days',
    '12h': 'Position — trades lasting 3-7 days',
    '24h': 'Macro — trades lasting 1-2 weeks',
};

export const getStatusConfig = (status?: string) => {
    if (!status) return statusConfig.APPROVED;
    return statusConfig[status] || statusConfig.APPROVED;
};
