export interface Prediction {
    id: string;
    symbol: string;
    direction: 'long' | 'short' | 'LONG' | 'SHORT' | 'NO_TRADE';
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
    riskDecision?: string;
    riskRewardRatio?: number;
    executionAction?: string;
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
    marketSignals?: string[];
    predictionReason?: string;
}
