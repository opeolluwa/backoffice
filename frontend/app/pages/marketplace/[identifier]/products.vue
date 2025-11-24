<template>
  <div class="space-y-4 grid grid-cols-12">
    <!-- <h1>Products for {{ identifier }} {{ marketplaceWithProducts }}</h1> -->
    <UPageCard
      v-for="value in marketplaceWithProducts?.products"
      :key="value.identifier"
      class="col-span-3"
    >
      <template #default>
        <div>
          <img :src="value.picture" class="size-10 rounded" />
          {{ value.name }}
          {{ value.description }}
          {{ value.price }}

        </div>
      </template>
    </UPageCard>
  </div>
</template>

<script lang="ts" setup>
import api from "~/plugin/api";
import type { MarketplaceWithProducts } from "../../../../../bindings/MarketplaceWithProducts";
const route = useRoute();
const identifier = route.params.identifier;

definePageMeta({
  layout: "dashboard",
});

const marketplaceWithProducts = ref<MarketplaceWithProducts>();

onMounted(async () => {
  try {
    const { data: response, status } = await api.get(
      `/marketplaces/${identifier}/products`,
    );
    if (status !== 200) {
      throw new Error(response?.message || "Login failed");
    }
    marketplaceWithProducts.value = response.data;
    return;
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
  } catch (error) {
    console.log({ error });
  }
});
</script>
