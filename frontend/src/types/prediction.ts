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
}
