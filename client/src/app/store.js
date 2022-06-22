import { configureStore } from '@reduxjs/toolkit';
import statSlice from '../features/stat/statSlice';

export const store = configureStore({
  reducer: {
    stat: statSlice,
  },
});
