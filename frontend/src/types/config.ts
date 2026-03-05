export interface Timeframe {
    value: string;
    label: string;
}

export interface AppConfig {
    pairs: string[];
    timeframes: Timeframe[];
}
