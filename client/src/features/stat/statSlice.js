import { createSlice } from '@reduxjs/toolkit';
import axios from 'axios';

const BASE_URL =
  process.env.node_env === 'production' ? '' : 'http://localhost:8080';

const initialState = {
  stats: null,
};

export const statSlice = createSlice({
  name: 'stat',
  initialState,
  // The `reducers` field lets us define reducers and generate associated actions
  reducers: {
    setStats: (state, action) => {
      state.stats = action.payload;
    },
  },
});

export const { setStats } = statSlice.actions;

const handleError = err => {
  console.error(err.toString());
};

export const getStatsAsync = () => dispatch => {
  return axios
    .get(`${BASE_URL}/stat/v1/`)
    .then(resp => resp.data)
    .then(stats => {
      dispatch(setStats(stats));
    })
    .catch(handleError);
};

export default statSlice.reducer;
