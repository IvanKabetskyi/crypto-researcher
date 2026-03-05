import { configureStore, combineReducers } from '@reduxjs/toolkit';
import { persistStore, persistReducer } from 'redux-persist';
import storage from 'redux-persist/lib/storage';
import signalsReducer from 'pages/Signals/slice';
import historyReducer from 'pages/History/slice';

const rootReducer = combineReducers({
    signals: signalsReducer,
    history: historyReducer,
});

const persistConfig = {
    key: 'cryptoResearcher0.0.3',
    storage,
    whitelist: [],
};

const persistedReducer = persistReducer(persistConfig, rootReducer);

export const store = configureStore({
    reducer: persistedReducer,
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware({
            serializableCheck: {
                ignoredActions: ['persist/PERSIST', 'persist/REHYDRATE'],
            },
        }),
});

export const persistor = persistStore(store);
