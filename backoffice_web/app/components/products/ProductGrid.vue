<script setup lang="ts">
import type { ProductsInterface } from "~/bindings/ProductsInterface";

defineProps<{
  products: ProductsInterface[];
  getPictureUrl: (uploadId: string) => string | null;
}>();
</script>

<template>
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
    <ProductsProductCard
      v-for="product in products"
      :key="product.identifier"
      :product="product"
      :picture-url="product.picture ? getPictureUrl(product.picture) : null"
    />
  </div>
</template>
