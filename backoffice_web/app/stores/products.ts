import { defineStore } from "pinia";
import api from "~/plugin/api";
import type { ProductsInterface as Product } from "~/bindings/ProductsInterface";

interface CreateProductPayload {
  picture?: string;
  name: string;
  description: string;
  price: number;
  currencyIdentifier: string;
}

const useProductStore = defineStore("products", {
  state: () => ({
    products: [] as Array<Product>,
    currentProduct: null as Product | null,
    count: 0,
  }),

  actions: {
    async fetchProducts() {
      const res = await api.get("/products");
      this.products = (res.data?.data as Array<Product>) || [];
      this.count = this.products.length;
    },

    async createProduct(payload: CreateProductPayload) {
      const res = await api.post("/products", payload);
      const created = res.data?.data as Product;
      this.products.unshift(created);
      this.count++;
      return created;
    },

    async findProduct(identifier: string) {
      const res = await api.get(`/products/${identifier}`);
      this.currentProduct = (res.data?.data as Product) || null;
      return this.currentProduct;
    },

    async deleteProduct(identifier: string) {
      await api.delete(`/products/${identifier}`);
      this.products = this.products.filter((p) => p.identifier !== identifier);
      this.count = Math.max(0, this.count - 1);
      if (this.currentProduct?.identifier === identifier)
        this.currentProduct = null;
    },
  },

  persist: true,
});

export { useProductStore };
export type { CreateProductPayload };
