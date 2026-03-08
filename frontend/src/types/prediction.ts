export interface Prediction {
    id: string;
    symbol: string;
    direction: 'long' | 'short';
    confidence: number;
    reasoning: string;
    entryPrice: number;
    targetPrice: number;
    stopLoss: number;
    createdAt: string;
    outcome: string | null;
    timeframe?: string;
    // Pipeline fields
    marketBias?: string;
    setupType?: string;
    riskDecision?: string;
    riskRewardRatio?: number;
    executionAction?: string;
    secondaryTarget?: number;
    invalidation?: number;
    positionSizePct?: number;
    reviewAgreed?: boolean;
    reviewConfidence?: number;
    reviewVerdict?: string;
    reviewDecision?: string;
    reviewIssues?: string[];
    reviewNotes?: string[];
    // Market context fields
    trendStrength?: 'weak' | 'moderate' | 'strong';
    momentum?: 'accelerating' | 'decelerating' | 'neutral';
    volumeProfile?: 'confirming' | 'weak' | 'divergent';
    derivativesSentiment?: 'bullish' | 'bearish' | 'neutral';
    predictionStatus?: string;
}
