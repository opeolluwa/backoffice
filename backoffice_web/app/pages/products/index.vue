<script setup lang="ts">
import { useProductStore } from "~/stores/products";
import { useCountryStore } from "~/stores/country";
import { useUploadStore } from "~/stores/uploads";

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
const uploadStore = useUploadStore();
const toast = useToast();

const isFetching = ref(true);
const openForm = ref(false);
const loading = ref(false);
const dialog = ref<InstanceType<any>>();

const hasProducts = computed(() => productStore.products.length > 0);

function getUploadUrl(uploadId: string): string | null {
  const upload = uploadStore.uploads.find((u) => u.identifier === uploadId);
  return upload?.url || null;
}

async function onSubmit(payload: {
  picture?: string;
  name: string;
  description: string;
  price: number;
  currencyIdentifier: string;
}) {
  loading.value = true;
  try {
    await productStore.createProduct(payload);
    dialog.value?.reset();
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
      uploadStore.findAllUploads(),
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

      <ProductsProductGrid
        :products="productStore.products"
        :get-picture-url="getUploadUrl"
      />
    </template>

    <ProductsCreateProductDialog
      ref="dialog"
      v-model:open="openForm"
      :loading="loading"
      @submit="onSubmit"
    />
  </div>
</template>
