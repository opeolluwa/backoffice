import axios from "axios";
import { useTokenStore } from "~/stores/token";

const api = axios.create({
  baseURL: "http://localhost:5006/api",
  headers: {
    Accept: "application/json",
  },
  timeout: 10000,
});

api.interceptors.request.use(
  (config) => {
    const tokenStore = useTokenStore();
    if (!tokenStore.isAccessTokenValid()) {
      // await tokenStore.getRefreshToken();
    }

    const token = tokenStore.accessToken;
    config.headers.Authorization = `Bearer ${token}`;
    return config;
  },
  (error) => Promise.reject(error),
);

// Response interceptor (handle errors globally)
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      console.warn("Unauthorized, maybe redirect to login");
    }
    return Promise.reject(error);
  },
);

export default api;
