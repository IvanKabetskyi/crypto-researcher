import { Provider } from 'react-redux';
import { PersistGate } from 'redux-persist/integration/react';
import { BrowserRouter } from 'react-router-dom';
import { ThemeProvider, CssBaseline, Box } from '@mui/material';
import { store, persistor } from 'store';
import { theme } from 'config/theme';
import { AppRouter } from 'router';
import { NavigationBar } from 'components/common/NavigationBar';

function App() {
    return (
        <Provider store={store}>
            <PersistGate loading={null} persistor={persistor}>
                <ThemeProvider theme={theme}>
                    <CssBaseline />
                    <BrowserRouter>
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
