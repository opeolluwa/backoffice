<script setup lang="ts">
import type { ProductsInterface } from "~/bindings/ProductsInterface";

defineProps<{
  product: ProductsInterface;
  pictureUrl: string | null;
}>();
</script>

<template>
  <div
    class="border border-gray-100 dark:border-white/5 rounded-2xl overflow-hidden hover:shadow-md transition"
  >
    <div
      class="aspect-video bg-gray-100 dark:bg-white/5 flex items-center justify-center overflow-hidden"
    >
      <img
        v-if="pictureUrl"
        :src="pictureUrl"
        :alt="product.name"
        class="w-full h-full object-cover"
      />
      <UIcon
        v-else
        name="heroicons:photo"
        class="size-10 text-gray-300 dark:text-white/20"
      />
    </div>

    <div class="p-4 space-y-2">
      <h3
        class="text-sm font-semibold text-gray-900 dark:text-white truncate"
      >
        {{ product.name }}
      </h3>
      <p class="text-xs text-gray-500 dark:text-white/40 line-clamp-2">
        {{ product.description }}
      </p>
      <div class="flex items-center justify-between pt-1">
        <span class="text-lg font-bold text-gray-900 dark:text-white">
          {{ product.price }}
        </span>
      </div>
    </div>
  </div>
</template>
