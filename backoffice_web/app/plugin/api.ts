// import axios, { type AxiosError, type InternalAxiosRequestConfig } from "axios";
// import { useTokenStore } from "~/stores/token";

// const NETWORK_REQUEST_TIMEOUT = 7500; // 7.5 seconds
// const UPLOADS_TIMEOUT = 300000; // 300 seconds
// const UPLOAD_LIMIT_SIZE = 1024 * 1024 * 25; // 25 MB

// const api = axios.create({
//   baseURL: "http://localhost:8000/api",
//   headers: {
//     Accept: "application/json",
//   },
//   timeout: NETWORK_REQUEST_TIMEOUT,
// });

// let isRefreshing = false;
// let failedQueue: Array<{
//   resolve: (value: unknown) => void;
//   reject: (reason?: unknown) => void;
// }> = [];

// const processQueue = (error: AxiosError | null) => {
//   failedQueue.forEach((prom) => {
//     if (error) {
//       prom.reject(error);
//     } else {
//       prom.resolve(undefined);
//     }
//   });
//   failedQueue = [];
// };

// api.interceptors.request.use(
//   (config) => {
//     const tokenStore = useTokenStore();
//     const token = tokenStore.accessToken;
//     if (token) {
//       config.headers.Authorization = `Bearer ${token}`;
//     }
//     return config;
//   },
//   (error) => Promise.reject(error),
// );

// const PUBLIC_AUTH_ENDPOINTS = ["/login", "/signup", "/forgotten-password", "/refresh-token"];

// api.interceptors.response.use(
//   (response) => response,
//   async (error: AxiosError) => {
//     const originalRequest = error.config as InternalAxiosRequestConfig & {
//       _retry?: boolean;
//     };

//     const isPublicAuthEndpoint = PUBLIC_AUTH_ENDPOINTS.some((path) =>
//       originalRequest.url?.includes(path),
//     );

//     if (
//       error.response?.status !== 401 ||
//       originalRequest._retry ||
//       isPublicAuthEndpoint
//     ) {
//       const message =
//         error.response?.data?.message || error.message || "Unknown error";
//       return Promise.reject(new Error(message));
//     }

//     const tokenStore = useTokenStore();

//     if (!tokenStore.isRefreshTokenValid()) {
//       tokenStore.clearTokens();
//       if (typeof window !== "undefined") {
//         window.location.href = "/";
//       }
//       return Promise.reject(error);
//     }

//     if (isRefreshing) {
//       return new Promise((resolve, reject) => {
//         failedQueue.push({ resolve, reject });
//       })
//         .then(() => api(originalRequest))
//         .catch((err) => Promise.reject(err));
//     }

//     originalRequest._retry = true;
//     isRefreshing = true;

//     try {
//       const success = await tokenStore.getRefreshToken();
//       if (success) {
//         processQueue(null);
//         return api(originalRequest);
//       } else {
//         processQueue(error);
//         tokenStore.clearTokens();
//         if (typeof window !== "undefined") {
//           window.location.href = "/";
//         }
//         return Promise.reject(error);
//       }
//     } catch (refreshError) {
//       processQueue(refreshError as AxiosError);
//       tokenStore.clearTokens();
//       if (typeof window !== "undefined") {
//         window.location.href = "/";
//       }
//       return Promise.reject(refreshError);
//     } finally {
//       isRefreshing = false;
//     }
//   },
// );

// export default api;
// export { UPLOADS_TIMEOUT, UPLOAD_LIMIT_SIZE };


import axios from "axios";
import { useTokenStore } from "~/stores/token";
const NETWORK_REQUEST_TIMEOUT = 7500; // 7.5 seconds
const UPLOADS_TIMEOUT = 300000; // 300 seconds
const UPLOAD_LIMIT_SIZE = 1024 * 1024 * 25; // 25 MB
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
export { UPLOADS_TIMEOUT, UPLOAD_LIMIT_SIZE };