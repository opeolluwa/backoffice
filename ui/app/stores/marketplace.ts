import api from "~/plugin/api";

const useMarketplaceStore = defineStore("marketplace", {
  state: () => ({
    marketplaces: [] as Array<{ id: number; name: string; slug: string }>,
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
});

export { useMarketplaceStore };
