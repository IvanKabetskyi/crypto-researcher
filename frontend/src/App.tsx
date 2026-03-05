import { Provider } from 'react-redux';
import { PersistGate } from 'redux-persist/integration/react';
import { BrowserRouter } from 'react-router-dom';
import { ThemeProvider, CssBaseline, Box } from '@mui/material';
import { store, persistor } from 'store';
import { theme } from './theme';
import { AppRouter } from './Router';
import { NavigationBar } from './NavigationBar';

function App() {
    return (
        <Provider store={store}>
            <PersistGate loading={null} persistor={persistor}>
                <ThemeProvider theme={theme}>
                    <CssBaseline />
                    <BrowserRouter basename={import.meta.env.BASE_URL}>
                        <Box
                            sx={{
                                minHeight: '100vh',
                                backgroundColor: 'background.default',
                            }}
                        >
                            <NavigationBar />
                            <AppRouter />
                        </Box>
                    </BrowserRouter>
                </ThemeProvider>
            </PersistGate>
        </Provider>
    );
}

export default App;
