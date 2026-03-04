import { createTheme } from '@mui/material';

export const theme = createTheme({
    palette: {
        mode: 'dark',
        primary: {
            main: '#00e676',
            light: '#66ffa6',
            dark: '#00b248',
        },
        secondary: {
            main: '#ff1744',
            light: '#ff616f',
            dark: '#c4001d',
        },
        background: {
            default: '#0a0e17',
            paper: '#131722',
        },
        text: {
            primary: '#e0e0e0',
            secondary: '#9e9e9e',
        },
    },
    typography: {
        fontFamily: '"Inter", "Roboto", "Helvetica", "Arial", sans-serif',
        h4: {
            fontWeight: 700,
        },
        h6: {
            fontWeight: 600,
        },
    },
    components: {
        MuiCard: {
            styleOverrides: {
                root: {
                    borderRadius: 12,
                    border: '1px solid rgba(255, 255, 255, 0.06)',
                },
            },
        },
        MuiChip: {
            styleOverrides: {
                root: {
                    fontWeight: 600,
                },
            },
        },
    },
});
