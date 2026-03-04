import React from 'react';
import { Card, CardContent, Typography } from '@mui/material';
import {
    BarChart,
    Bar,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    ResponsiveContainer,
    Cell,
} from 'recharts';
import { SymbolAccuracy } from 'core/Entities/Accuracy/types';

interface AccuracyChartProps {
    bySymbol: Record<string, SymbolAccuracy>;
}

export const AccuracyChart: React.FC<AccuracyChartProps> = ({ bySymbol }) => {
    const data = Object.entries(bySymbol).map(([symbol, stats]) => ({
        symbol,
        accuracy: stats.accuracy,
        total: stats.total,
        correct: stats.correct,
    }));

    return (
        <Card sx={{ backgroundColor: 'background.paper' }}>
            <CardContent>
                <Typography variant="h6" mb={2}>
                    Accuracy by Symbol
                </Typography>
                <ResponsiveContainer width="100%" height={300}>
                    <BarChart data={data}>
                        <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.06)" />
                        <XAxis dataKey="symbol" stroke="#9e9e9e" fontSize={12} />
                        <YAxis stroke="#9e9e9e" fontSize={12} domain={[0, 100]} />
                        <Tooltip
                            contentStyle={{
                                backgroundColor: '#1a1f2e',
                                border: '1px solid rgba(255,255,255,0.1)',
                                borderRadius: 8,
                            }}
                            // eslint-disable-next-line @typescript-eslint/no-explicit-any
                            formatter={((value: any, name: any) => [
                                typeof value === 'number' ? `${value.toFixed(1)}%` : '—',
                                name === 'accuracy' ? 'Accuracy' : String(name ?? ''),
                            ]) as any}
                        />
                        <Bar dataKey="accuracy" radius={[4, 4, 0, 0]}>
                            {data.map((entry, index) => (
                                <Cell
                                    key={index}
                                    fill={entry.accuracy >= 80 ? '#00e676' : entry.accuracy >= 60 ? '#ffab00' : '#ff1744'}
                                />
                            ))}
                        </Bar>
                    </BarChart>
                </ResponsiveContainer>
            </CardContent>
        </Card>
    );
};
