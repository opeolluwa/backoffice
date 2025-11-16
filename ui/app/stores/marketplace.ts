import api from "~/plugin/api";
import type { MarketPlace } from "~/types/Marketplace";

const useMarketplaceStore = defineStore("marketplace", {
  state: () => ({
    marketplaces: [] as Array<MarketPlace>,
  }),

  actions: {
    async fetchMarketplaces() {
      try {
        const res = await api.get("/marketplaces");
        this.marketplaces = res.data?.data || [];
      } catch (error) {
        console.error("Failed to fetch marketplaces:", error);
      }
    },
  },
  persist: true,  
});

export { useMarketplaceStore };
