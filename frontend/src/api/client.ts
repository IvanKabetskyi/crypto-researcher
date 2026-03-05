import axios from 'axios';

declare const __API_URL__: string;

const apiClient = axios.create({
    baseURL: __API_URL__,
    timeout: 10000,
    headers: {
        'Content-Type': 'application/json',
    },
});

apiClient.interceptors.request.use((config) => {
    const token = localStorage.getItem('token');
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
});

apiClient.interceptors.response.use(
    (response) => response,
    (error) => {
        const isLoginRequest = error.config?.url?.includes('/auth/login');
        if (error.response?.status === 401 && !isLoginRequest) {
            localStorage.removeItem('token');
            localStorage.removeItem('email');
            window.location.href = import.meta.env.BASE_URL + 'login';
        }
        return Promise.reject(error);
    },
);

export default apiClient;
