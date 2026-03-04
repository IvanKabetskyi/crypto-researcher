import axios from 'axios';

declare const __API_URL__: string;

const apiClient = axios.create({
    baseURL: __API_URL__,
    timeout: 10000,
    headers: {
        'Content-Type': 'application/json',
    },
});

export default apiClient;
