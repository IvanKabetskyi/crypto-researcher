import { AppBar, Toolbar, Typography, Button, Box } from '@mui/material';
import ShowChartIcon from '@mui/icons-material/ShowChart';
import HistoryIcon from '@mui/icons-material/History';
import SettingsIcon from '@mui/icons-material/Settings';
import { useNavigate, useLocation } from 'react-router-dom';

export const NavigationBar = () => {
    const navigate = useNavigate();
    const location = useLocation();

    return (
        <AppBar
            position="static"
            sx={{
                backgroundColor: 'background.paper',
                borderBottom: '1px solid rgba(255,255,255,0.06)',
            }}
            elevation={0}
        >
            <Toolbar>
                <ShowChartIcon sx={{ mr: 1, color: 'primary.main' }} />
                <Typography variant="h6" fontWeight={700} sx={{ flexGrow: 0, mr: 4 }}>
                    Crypto Researcher
                </Typography>

                <Box sx={{ display: 'flex', gap: 1, flexGrow: 1 }}>
                    <Button
                        color={location.pathname === '/' ? 'primary' : 'inherit'}
                        onClick={() => navigate('/')}
                        sx={{ fontWeight: location.pathname === '/' ? 700 : 400 }}
                    >
                        Dashboard
                    </Button>
                    <Button
                        color={location.pathname === '/analytics' ? 'primary' : 'inherit'}
                        onClick={() => navigate('/analytics')}
                        sx={{ fontWeight: location.pathname === '/analytics' ? 700 : 400 }}
                    >
                        Analytics
                    </Button>
                    <Button
                        color={location.pathname === '/history' ? 'primary' : 'inherit'}
                        onClick={() => navigate('/history')}
                        startIcon={<HistoryIcon />}
                        sx={{ fontWeight: location.pathname === '/history' ? 700 : 400 }}
                    >
                        History
                    </Button>
                </Box>

                <Button
                    color={location.pathname === '/settings' ? 'primary' : 'inherit'}
                    onClick={() => navigate('/settings')}
                    startIcon={<SettingsIcon />}
                    sx={{ fontWeight: location.pathname === '/settings' ? 700 : 400 }}
                >
                    Settings
                </Button>
            </Toolbar>
        </AppBar>
    );
};
