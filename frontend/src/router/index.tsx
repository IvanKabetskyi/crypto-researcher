import { Routes, Route } from 'react-router-dom';
import { Dashboard } from 'pages/Dashboard';
import { Analytics } from 'pages/Analytics';
import { History } from 'pages/History';
import { Settings } from 'pages/Settings';

export const AppRouter = () => {
    return (
        <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/analytics" element={<Analytics />} />
            <Route path="/history" element={<History />} />
            <Route path="/settings" element={<Settings />} />
        </Routes>
    );
};
