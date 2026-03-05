import { Routes, Route, Navigate } from 'react-router-dom';
import { Signals } from 'pages/Signals';
import { History } from 'pages/History';
import { Login } from 'pages/Login';

const ProtectedRoute = ({ children }: { children: React.ReactNode }) => {
    const token = localStorage.getItem('token');
    if (!token) {
        return <Navigate to="/login" replace />;
    }
    return <>{children}</>;
};

export const AppRouter = () => {
    return (
        <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/" element={<ProtectedRoute><Signals /></ProtectedRoute>} />
            <Route path="/history" element={<ProtectedRoute><History /></ProtectedRoute>} />
        </Routes>
    );
};
