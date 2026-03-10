export interface Timeframe {
    value: string;
    label: string;
    description: string;
}

export interface AppConfig {
    pairs: string[];
    timeframes: Timeframe[];
}
