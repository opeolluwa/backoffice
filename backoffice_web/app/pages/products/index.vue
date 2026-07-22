<script setup lang="ts">
import { useProductStore } from "~/stores/products";
import { useCountryStore } from "~/stores/country";
import { z } from "zod";

useHead({ title: "Products" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:cube",
    ariaLabel: "Products",
    title: "Products",
  },
});

const productStore = useProductStore();
const countryStore = useCountryStore();
const toast = useToast();

const isFetching = ref(true);
const openForm = ref(false);
const loading = ref(false);

const hasProducts = computed(() => productStore.products.length > 0);

const state = reactive({
  name: "",
  description: "",
  price: 0,
  picture: null as File | null,
  currencyIdentifier: "",
});

const schema = z.object({
  name: z.string().min(1, "Name is required"),
  description: z.string().min(1, "Description is required"),
  price: z.number().min(1, "Price is required"),
  picture: z.any().optional(),
  currencyIdentifier: z.string().min(1, "Currency is required"),
});

const currencyOptions = computed(() =>
  countryStore.countries.map((c) => ({
    label: `${c.currency} (${c.country})`,
    value: c.identifier,
    avatar: c.flag ? { src: c.flag } : undefined,
  })),
);

function resetForm() {
  state.name = "";
  state.description = "";
  state.price = 0;
  state.picture = null;
  state.currencyIdentifier = "";
}

async function onSubmit() {
  loading.value = true;
  try {
    const formData = new FormData();
    formData.append("name", state.name);
    formData.append("description", state.description);
    formData.append("price", state.price.toString());
    formData.append("currencyIdentifier", state.currencyIdentifier);
    if (state.picture) {
      formData.append("picture", state.picture);
    }

    await productStore.createProduct(formData);
    resetForm();
    openForm.value = false;
    toast.add({ title: "Product created", color: "success" });
  } catch (error: any) {
    toast.add({
      title: "Failed to create product",
      description: error?.message || "Please try again.",
      color: "error",
    });
  } finally {
    loading.value = false;
  }
}

onMounted(async () => {
  try {
    await Promise.all([
      productStore.fetchProducts(),
      countryStore.fetchCountries(),
    ]);
  } catch (error) {
    console.error(error);
  } finally {
    isFetching.value = false;
  }
});
</script>

<template>
  <div class="space-y-6">
    <PageLoader v-if="isFetching" />

    <AppEmptyState
      v-else-if="!hasProducts"
      icon="heroicons:cube"
      title="No products yet"
      description="Create your first product to get started."
      action-label="Create product"
      @action="openForm = true"
    />

    <template v-else>
      <AppPageHeader
        title="Products"
        subtitle="Manage your products"
        cta-text="Create product"
        @cta="openForm = true"
      />

      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        <div
          v-for="product in productStore.products"
          :key="product.identifier"
          class="border border-gray-100 dark:border-white/5 rounded-2xl overflow-hidden hover:shadow-md transition"
        >
          <div class="aspect-video bg-gray-100 dark:bg-white/5 flex items-center justify-center overflow-hidden">
            <img
              v-if="product.picture"
              :src="product.picture"
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
            <h3 class="text-sm font-semibold text-gray-900 dark:text-white truncate">
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
      </div>
    </template>

    <UModal
      v-model:open="openForm"
      title="Create product"
      description="Add a new product to your catalog"
      close-icon="heroicons:x-mark"
    >
      <template #body>
        <UForm
          class="space-y-4"
          :schema="schema"
          :state="state"
          :on-submit="onSubmit"
        >
          <UFileUpload v-model="state.picture" class="w-full min-h-48" />

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

          <div class="flex justify-between gap-x-2">
            <UFormField
              v-slot="{ error }"
              label="Price"
              name="price"
              required
              :ui="{ error: 'text-red-500 text-sm mt-1' }"
            >
              <UInputNumber
                v-model="state.price"
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
              label="Currency"
              name="currencyIdentifier"
              required
              class="w-full"
              :ui="{ error: 'text-red-500 text-sm mt-1' }"
            >
              <USelect
                v-model="state.currencyIdentifier"
                :items="currencyOptions"
                :ui="{ base: 'py-4 px-6', viewport: '', content: 'w-60' }"
                :class="[
                  'w-full transition-colors',
                  error
                    ? 'border-red-500 focus:border-red-500'
                    : 'border-gray-300 focus:border-black',
                ]"
              />
            </UFormField>
          </div>

          <div class="flex justify-between items-center">
            <UButton
              type="submit"
              class="dark:text-white/90 py-3 px-4"
              :loading="loading"
              :disabled="loading"
            >
              Create
            </UButton>
            <UButton
              variant="subtle"
              class="dark:text-white/90 py-3 px-4"
              @click="resetForm"
            >
              Clear form
            </UButton>
          </div>
        </UForm>
      </template>
    </UModal>
  </div>
</template>
