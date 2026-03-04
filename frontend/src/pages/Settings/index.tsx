import { useState, useEffect } from 'react';
import {
    Box,
    Typography,
    Card,
    CardContent,
    TextField,
    Button,
    Alert,
    Snackbar,
    CircularProgress,
    Chip,
} from '@mui/material';
import SaveIcon from '@mui/icons-material/Save';
import { predictionRequests, Settings as SettingsType, SettingsPayload } from 'core/Gateways/Prediction/requests';

export const Settings = () => {
    const [settings, setSettings] = useState<SettingsType | null>(null);
    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [snackbar, setSnackbar] = useState<{ open: boolean; message: string; severity: 'success' | 'error' }>({
        open: false,
        message: '',
        severity: 'success',
    });

    const [form, setForm] = useState<SettingsPayload>({
        ai_model: '',
        watch_pairs: '',
        analysis_interval_secs: 300,
    });

    useEffect(() => {
        loadSettings();
    }, []);

    const loadSettings = async () => {
        try {
            setLoading(true);
            const data = await predictionRequests.fetchSettings();
            setSettings(data);
            setForm({
                ai_model: data.ai_model,
                watch_pairs: data.watch_pairs,
                analysis_interval_secs: parseInt(data.analysis_interval_secs) || 300,
            });
        } catch {
            setError('Cannot connect to backend. Make sure the Rust server is running on port 8080.');
        } finally {
            setLoading(false);
        }
    };

    const handleSave = async () => {
        try {
            setSaving(true);
            await predictionRequests.updateSettings({ ...form });
            setSnackbar({ open: true, message: 'Settings saved successfully!', severity: 'success' });
            await loadSettings();
        } catch {
            setSnackbar({ open: true, message: 'Failed to save settings', severity: 'error' });
        } finally {
            setSaving(false);
        }
    };

    if (loading) {
        return (
            <Box display="flex" justifyContent="center" alignItems="center" minHeight={400}>
                <CircularProgress color="primary" />
            </Box>
        );
    }

    return (
        <Box sx={{ p: 3, maxWidth: 700, mx: 'auto' }}>
            <Typography variant="h4" mb={3}>
                Settings
            </Typography>

            {error && (
                <Alert severity="error" sx={{ mb: 3 }}>
                    {error}
                </Alert>
            )}

            <Card sx={{ backgroundColor: 'background.paper', mb: 3 }}>
                <CardContent>
                    <Typography variant="h6" mb={2}>
                        AI Provider
                    </Typography>
                    <Alert severity="info" sx={{ mb: 2 }}>
                        Using Groq API (free). Get your key at console.groq.com
                    </Alert>

                    <Box mb={2}>
                        <Box display="flex" alignItems="center" gap={1} mb={0.5}>
                            <Typography variant="body2" fontWeight={600}>
                                API URL
                            </Typography>
                            <Chip
                                label={settings?.ai_url || 'https://api.groq.com/openai/v1'}
                                size="small"
                                color="success"
                                variant="outlined"
                            />
                        </Box>
                        <Typography variant="body2" color="text.secondary">
                            Set AI_API_KEY in backend .env file to connect.
                        </Typography>
                    </Box>

                    <Box mb={2}>
                        <Typography variant="body2" fontWeight={600} mb={0.5}>
                            AI Model
                        </Typography>
                        <TextField
                            fullWidth
                            size="small"
                            value={form.ai_model || ''}
                            onChange={(e) => setForm({ ...form, ai_model: e.target.value })}
                            helperText="Groq model (e.g. llama-3.3-70b-versatile, mixtral-8x7b-32768)"
                        />
                    </Box>

                    <Box mb={1}>
                        <Box display="flex" alignItems="center" gap={1} mb={0.5}>
                            <Typography variant="body2" fontWeight={600}>
                                News Source
                            </Typography>
                            <Chip
                                label={settings?.news_source || 'Free RSS feeds'}
                                size="small"
                                color="success"
                                variant="outlined"
                            />
                        </Box>
                        <Typography variant="body2" color="text.secondary">
                            Using free RSS feeds from CoinDesk and CoinTelegraph - no API key needed.
                        </Typography>
                    </Box>
                </CardContent>
            </Card>

            <Card sx={{ backgroundColor: 'background.paper', mb: 3 }}>
                <CardContent>
                    <Typography variant="h6" mb={2}>
                        Analysis Configuration
                    </Typography>

                    <Box mb={2}>
                        <Typography variant="body2" fontWeight={600} mb={0.5}>
                            Watch Pairs
                        </Typography>
                        <TextField
                            fullWidth
                            size="small"
                            value={form.watch_pairs || ''}
                            onChange={(e) => setForm({ ...form, watch_pairs: e.target.value })}
                            helperText="Comma-separated Bybit trading pairs (e.g. BTCUSDT,ETHUSDT,SOLUSDT)"
                        />
                    </Box>

                    <Box mb={1}>
                        <Typography variant="body2" fontWeight={600} mb={0.5}>
                            Analysis Interval (seconds)
                        </Typography>
                        <TextField
                            fullWidth
                            size="small"
                            type="number"
                            value={form.analysis_interval_secs || 300}
                            onChange={(e) =>
                                setForm({ ...form, analysis_interval_secs: parseInt(e.target.value) || 300 })
                            }
                            helperText="How often to run automatic analysis (default: 300 = 5 minutes)"
                        />
                    </Box>
                </CardContent>
            </Card>

            <Button
                variant="contained"
                startIcon={saving ? <CircularProgress size={20} color="inherit" /> : <SaveIcon />}
                onClick={handleSave}
                disabled={saving}
                fullWidth
                size="large"
                sx={{ fontWeight: 700 }}
            >
                {saving ? 'Saving...' : 'Save Settings'}
            </Button>

            <Snackbar
                open={snackbar.open}
                autoHideDuration={4000}
                onClose={() => setSnackbar({ ...snackbar, open: false })}
            >
                <Alert severity={snackbar.severity} onClose={() => setSnackbar({ ...snackbar, open: false })}>
                    {snackbar.message}
                </Alert>
            </Snackbar>
        </Box>
    );
};
