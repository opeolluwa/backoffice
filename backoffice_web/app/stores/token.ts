import { defineStore } from "pinia";
import api from "~/plugin/api";

export const useTokenStore = defineStore("token_store", {
  state: () => ({
    accessToken: "",
    refreshToken: "",
    accessTokenExpiry: 0,
    refreshTokenExpiry: 0,
    isRefreshing: false,
  }),

  actions: {
    persistRefreshToken(refreshToken: string) {
      this.refreshToken = refreshToken;
    },

    persistAccessToken(accessToken: string) {
      this.accessToken = accessToken;
    },

    clearTokens() {
      this.$reset();
    },

    setAccessTokenExpiry(expiry: number) {
      this.accessTokenExpiry = expiry;
    },

    setRefreshTokenExpiry(expiry: number) {
      this.refreshTokenExpiry = expiry;
    },

    extractAccessToken() {
      return this.accessToken;
    },

    async getRefreshToken(): Promise<boolean> {
      if (this.isRefreshing) return false;
      if (!this.refreshToken) return false;

      this.isRefreshing = true;

      try {
        const res = await api.post("/refresh-token", {
          refreshToken: this.refreshToken,
        });

        const data = res.data?.data;
        if (!data) return false;

        this.persistAccessToken(data.accessToken);
        this.setAccessTokenExpiry(data.accessTokenExpiry);
        this.persistRefreshToken(data.refreshToken);
        this.setRefreshTokenExpiry(data.refreshTokenExpiry);

        return true;
      } catch {
        this.clearTokens();
        return false;
      } finally {
        this.isRefreshing = false;
      }
    },

    isAccessTokenValid() {
      const now = Math.floor(Date.now() / 1000);
      return this.accessToken && this.accessTokenExpiry > now + 60;
    },

    isRefreshTokenValid() {
      const now = Math.floor(Date.now() / 1000);
      return this.refreshToken && this.refreshTokenExpiry > now + 60;
    },
  },

  getters: {},

  persist: true,
});
