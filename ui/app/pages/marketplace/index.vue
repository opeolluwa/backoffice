<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";
import api from "~/plugin/api";
import { useMarketplaceStore } from "~/stores/marketplace";

definePageMeta({
  layout: "dashboard",
});

const schema = v.object({
  name: v.pipe(v.string(), v.minLength(1, "Name is required ")),
  description: v.pipe(v.string(), v.minLength(1, "Description is required ")),
  slug: v.pipe(v.string(), v.minLength(1, "Slug is required ")),
});

type Schema = v.InferOutput<typeof schema>;

const openForm = ref(false);
const toast = useToast();
const state = reactive<Schema>({
  name: "",
  description: "",
  slug: "",
});

const resetForm = () => {
  state.name = "";
  state.description = "";
  state.slug = "";
};

const marketplaceStore = useMarketplaceStore();
const fetchingMarketplaces = ref(false);

const loading = ref(false);
async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    const res = await api.post("/marketplace", data);
    if (res.status !== 201) {
      throw new Error(res.data?.message || "Failed to create marketplace");
    }
    toast.add({
      title: "Success",
      description: "Product added successfully.",
    });
    openForm.value = false;
    resetForm();
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to create marketplace. Please try again.",
      color: "error",
    });
  } finally {
    loading.value = false;
  }
}

onMounted(async () => {
  try {
    fetchingMarketplaces.value = true;
    const res = await api.get("/marketplace");
    if (res.status !== 200) {
      throw new Error(res.data?.message || "Failed to fetch marketplaces");
    }
    marketplaceStore.marketplaces = res.data.data.marketplaces;
    console.log(
      "Marketplaces loaded:",
      JSON.stringify(marketplaceStore.marketplaces, null, 2),
    );
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to load marketplaces. Please try again.",
      color: "error",
    });
  } finally {
    fetchingMarketplaces.value = false;
  }
});
</script>

<template>
  <div>

     <div
      v-if="marketplaceStore.marketplaces?.length === 0"
      class="flex flex-col justify-center items-center h-[70vh]"
    >
      <h1>You currently don&apos;t have any product</h1>

      <UButton
        color="neutral"
        variant="outline"
        class="px-5 py-4 rounded mt-3 cursor-pointer flex items-center justify-center gap-x-2"
        @click="openForm = true"
      >
        <UIcon name="heroicons:plus-circle" class="size-5" />
        Add product
      </UButton>
    </div>

    <!-- {{ marketplaceStore.marketplaces }}
    <PageLoader v-if="fetchingMarketplaces" />

    <div v-else>
      <div
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6"
      >
        <div
          v-for="marketplace in marketplaceStore.marketplaces"
          :key="marketplace.identifier"
          class="border border-gray-200 rounded-lg p-4 flex flex-col justify-between"
        >
          <div>
            <h2 class="text-lg font-semibold mb-2">{{ marketplace.name }}</h2>
            <p class="text-gray-600 mb-4">{{ marketplace.description }}</p>
          </div>
          <div>
            <span class="text-sm text-gray-500"
              >Slug: {{ marketplace.slug }}</span
            >
          </div>
        </div>
      </div>
    </div> -->

   

    <UModal
      v-model:open="openForm"
      title="Create store"
      description="A store lets you manage your goods"
      close-icon="heroicons:x-mark"
    >
      <template #body>
        <UForm
          class="space-y-4"
          :schema="schema"
          :state="state"
          @submit.prevent="onSubmit"
        >
          <UFormField
            v-slot="{ error }"
            label="Name"
            name="name"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          >
            <UInput
              v-model="state.name"
              :ui="{ base: 'py-4 px-6' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <UFormField
            v-slot="{ error }"
            label="Description"
            name="description"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          >
            <UInput
              v-model="state.description"
              :ui="{ base: 'py-4 px-6' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <UFormField
            v-slot="{ error }"
            label="Slug"
            name="slug"
            hint="eg: cars, roses"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          >
            <UInput
              v-model="state.slug"
              :ui="{ base: 'py-4 px-6' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <div class="flex justify-between items-center">
            <UButton type="submit" :loading="loading" :disabled="loading">
              Continue
            </UButton>
            <UButton variant="subtle" color="error" @click="resetForm">
              Clear form
            </UButton>
          </div>
        </UForm>
      </template>
    </UModal>
  </div>
</template>

<style scoped></style>
