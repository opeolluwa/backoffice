import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { AppConfigInterface } from "~/bindings/AppConfigInterface";

export const useAppStore = defineStore("app", {
  state: () => ({
    config: null as AppConfigInterface | null,
  }),

  actions: {
    async fetchConfig() {
      try {
        const res = await api.get("/config");
        this.config = res.data?.data || null;

        if (!this.config) {
          const created = await api.post("/config", {});
          this.config = created.data?.data || null;
        }

        return this.config;
      } catch (error) {
        console.error("Failed to fetch app config:", error);
        return null;
      }
    },

    async updateConfig(payload: {
      defaultCurrency?: string | null;
      defaultLanguage?: string | null;
    }) {
      try {
        const res = await api.put("/config", payload);
        this.config = res.data?.data || null;
        return this.config;
      } catch (error) {
        console.error("Failed to update app config:", error);
        throw error;
      }
    },
  },

  persist: true,
});
