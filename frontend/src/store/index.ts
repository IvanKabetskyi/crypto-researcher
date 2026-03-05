import { configureStore, combineReducers } from '@reduxjs/toolkit';
import { persistStore, persistReducer } from 'redux-persist';
import storage from 'redux-persist/lib/storage';
import signalsReducer from 'pages/Signals/redux';
import historyReducer from 'pages/History/redux';
import filterReducer from 'store/slices/Filter';

const rootReducer = combineReducers({
    signals: signalsReducer,
    history: historyReducer,
    filter: filterReducer,
});

const persistConfig = {
    key: 'cryptoResearcher0.0.2',
    storage,
    whitelist: ['filter'],
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
