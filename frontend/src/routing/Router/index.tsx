import { Routes, Route } from 'react-router-dom';
import { Signals } from 'pages/Signals';
import { History } from 'pages/History';

export const AppRouter = () => {
    return (
        <Routes>
            <Route path="/" element={<Signals />} />
            <Route path="/history" element={<History />} />
        </Routes>
    );
};
