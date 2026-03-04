import axios from 'axios';

const getApiUrl = (): string => {
    const config = (window as any).__APP_CONFIG__;
    if (config?.API_URL && config.API_URL !== '__API_URL__') {
        return config.API_URL;
    }
    return 'http://localhost:8080/api';
};

const apiClient = axios.create({
    baseURL: getApiUrl(),
    timeout: 10000,
    headers: {
        'Content-Type': 'application/json',
    },
});

export default apiClient;
