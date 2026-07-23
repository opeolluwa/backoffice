import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { CustomersInterface as Customer } from "~/bindings/CustomersInterface";

const useCustomersStore = defineStore("customers", {
  state: () => ({
    customers: [] as Array<Customer>,
    currentCustomer: null as Customer | null,
    count: 0,
  }),

  actions: {
    async fetchCustomers() {
      const res = await api.get("/customers");
      this.customers = (res.data?.data as Array<Customer>) || [];
      this.count = this.customers.length;
    },

    async findCustomer(identifier: string) {
      const res = await api.get(`/customers/${identifier}`);
      this.currentCustomer = (res.data?.data as Customer) || null;
      return this.currentCustomer;
    },

    async countCustomers() {
      const res = await api.get("/customers/count");
      this.count = res.data?.data ?? 0;
    },

    async deleteCustomer(identifier: string) {
      await api.delete(`/customers/${identifier}`);
      this.customers = this.customers.filter(
        (c) => c.identifier !== identifier,
      );
      this.count = Math.max(0, this.count - 1);
      if (this.currentCustomer?.identifier === identifier)
        this.currentCustomer = null;
    },
  },

  persist: true,
});

export { useCustomersStore };
