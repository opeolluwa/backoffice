import axios from "axios";
import { useTokenStore } from "~/stores/token";
const NETWORK_REQUEST_TIMEOUT = 7500; // 7.5 seconds
const UPLOADS_TIMEOUT = 300000; // 300 seconds
const api = axios.create({
  baseURL: "http://localhost:8000/api",
  headers: {
    Accept: "application/json",
  },
  timeout: NETWORK_REQUEST_TIMEOUT,
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
    const message =
      error.response?.data?.message || error.message || "Unknown error";

    return Promise.reject(new Error(message));
  },
);

export default api;
export { UPLOADS_TIMEOUT };
