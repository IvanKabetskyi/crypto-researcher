import { Prediction } from 'core/Entities/Prediction/types';

export interface HistoryResponse {
    items: Prediction[];
    total: number;
    page: number;
    perPage: number;
    totalPages: number;
}

export interface HistoryParams {
    symbol?: string;
    direction?: string;
    outcome?: string;
    date_from?: string;
    date_to?: string;
    page?: number;
    per_page?: number;
}
