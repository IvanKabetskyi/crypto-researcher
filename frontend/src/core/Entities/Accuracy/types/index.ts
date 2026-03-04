export interface SymbolAccuracy {
    total: number;
    correct: number;
    accuracy: number;
}

export interface AccuracyStats {
    totalPredictions: number;
    correct: number;
    incorrect: number;
    pending: number;
    accuracyPercentage: number;
    bySymbol: Record<string, SymbolAccuracy>;
}
