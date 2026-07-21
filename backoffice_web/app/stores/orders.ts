import api from "~/plugin/api";
import type { OrdersInterface } from "~/bindings/OrdersInterface";

const useOrdersStore = defineStore("orders", {
  state: () => ({
    orders: [] as Array<OrdersInterface>,
  }),

  actions: {
    async fetchOrders() {
      try {
        const res = await api.get("/orders");
        this.orders = res.data?.data || [];
      } catch (error) {
        console.error("Failed to fetch orders:", error);
      }
    },
  },
  persist: true,
});

export { useOrdersStore };
