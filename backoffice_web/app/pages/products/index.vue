<script setup lang="ts">
import { useProductStore } from "~/stores/products";
import { useCountryStore } from "~/stores/country";
import { useUploadStore } from "~/stores/uploads";
import type { UploadsInterface } from "~/bindings/UploadsInterface";
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
const uploadStore = useUploadStore();
const toast = useToast();

const isFetching = ref(true);
const openForm = ref(false);
const loading = ref(false);

type ImageSource = "upload" | "library";
const imageSource = ref<ImageSource>("upload");
const newFile = ref<File | null>(null);
const selectedUpload = ref<UploadsInterface | null>(null);

const hasProducts = computed(() => productStore.products.length > 0);

const imageUploads = computed(() =>
  uploadStore.uploads.filter((u) => u.fileType === "Image"),
);

const state = reactive({
  name: "",
  description: "",
  price: 0,
  currencyIdentifier: "",
});

const schema = z.object({
  name: z.string().min(1, "Name is required"),
  description: z.string().min(1, "Description is required"),
  price: z.number().min(1, "Price is required"),
  currencyIdentifier: z.string().min(1, "Currency is required"),
});

const currencyOptions = computed(() =>
  countryStore.countries.map((c) => ({
    label: `${c.currencyCode} ${c.country}`,
    avatar: c.flag ? { src: c.flag } : undefined,
    value: c.identifier,
  })),
);

function getUploadUrl(uploadId: string): string | null {
  const upload = uploadStore.uploads.find((u) => u.identifier === uploadId);
  return upload?.url || null;
}

function resetForm() {
  state.name = "";
  state.description = "";
  state.price = 0;
  state.currencyIdentifier = "";
  newFile.value = null;
  selectedUpload.value = null;
  imageSource.value = "upload";
}

function selectUpload(upload: UploadsInterface) {
  selectedUpload.value =
    selectedUpload.value?.identifier === upload.identifier ? null : upload;
}

async function onSubmit() {
  loading.value = true;
  try {
    let pictureId: string | undefined;

    if (imageSource.value === "upload" && newFile.value) {
      const formData = new FormData();
      formData.append("file", newFile.value);
      formData.append("name", newFile.value.name);
      const created = await uploadStore.createUpload({
        file: newFile.value,
        name: newFile.value.name,
      });
      if (created && "identifier" in created) {
        pictureId = created.identifier;
      }
    } else if (imageSource.value === "library" && selectedUpload.value) {
      pictureId = selectedUpload.value.identifier;
    }

    await productStore.createProduct({
      picture: pictureId,
      name: state.name,
      description: state.description,
      price: state.price,
      currencyIdentifier: state.currencyIdentifier,
    });
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

      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        <div
          v-for="product in productStore.products"
          :key="product.identifier"
          class="border border-gray-100 dark:border-white/5 rounded-2xl overflow-hidden hover:shadow-md transition"
        >
          <div class="aspect-video bg-gray-100 dark:bg-white/5 flex items-center justify-center overflow-hidden">
            <img
              v-if="product.picture && getUploadUrl(product.picture)"
              :src="getUploadUrl(product.picture)"
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
          <div class="space-y-3">
            <label class="text-xs font-medium text-gray-600 dark:text-gray-400">
              Image
            </label>
            <div class="flex gap-2">
              <button
                type="button"
                class="px-3 py-1.5 text-xs font-medium rounded-lg transition-colors cursor-pointer"
                :class="
                  imageSource === 'upload'
                    ? 'bg-black text-white dark:bg-white dark:text-black'
                    : 'bg-gray-100 dark:bg-white/5 text-gray-600 dark:text-white/40 hover:bg-gray-200 dark:hover:bg-white/10'
                "
                @click="imageSource = 'upload'"
              >
                Upload new
              </button>
              <button
                type="button"
                class="px-3 py-1.5 text-xs font-medium rounded-lg transition-colors cursor-pointer"
                :class="
                  imageSource === 'library'
                    ? 'bg-black text-white dark:bg-white dark:text-black'
                    : 'bg-gray-100 dark:bg-white/5 text-gray-600 dark:text-white/40 hover:bg-gray-200 dark:hover:bg-white/10'
                "
                @click="imageSource = 'library'"
              >
                Choose from library
              </button>
            </div>

            <div v-if="imageSource === 'upload'">
              <UFileUpload v-model="newFile" class="w-full min-h-48" />
            </div>

            <div v-else>
              <div
                v-if="imageUploads.length === 0"
                class="text-sm text-gray-400 dark:text-white/30 text-center py-8"
              >
                No images in library. Upload some first.
              </div>
              <div
                v-else
                class="grid grid-cols-3 sm:grid-cols-4 gap-3 max-h-64 overflow-y-auto"
              >
                <button
                  v-for="upload in imageUploads"
                  :key="upload.identifier"
                  type="button"
                  class="aspect-square rounded-xl overflow-hidden border-2 transition-all cursor-pointer"
                  :class="
                    selectedUpload?.identifier === upload.identifier
                      ? 'border-black dark:border-white ring-2 ring-black/20 dark:ring-white/20'
                      : 'border-transparent hover:border-gray-300 dark:hover:border-white/20'
                  "
                  @click="selectUpload(upload)"
                >
                  <img
                    :src="upload.url"
                    :alt="upload.name"
                    class="w-full h-full object-cover"
                  />
                </button>
              </div>
            </div>
          </div>

          <AppInput
            v-model="state.name"
            label="Name"
            name="name"
            placeholder="Product name"
          />

          <AppInput
            v-model="state.description"
            label="Description"
            name="description"
            placeholder="Product description"
          />

          <div class="flex justify-between gap-x-2">
            <AppNumberInput
              v-model="state.price"
              label="Price"
              name="price"
              placeholder="0"
              class="w-full"
            />
            <AppSelect
              v-model="state.currencyIdentifier"
              label="Currency"
              name="currencyIdentifier"
              :items="currencyOptions"
              placeholder="Select currency"
              required
              class="w-full"
            />
          </div>

          <div class="flex justify-between items-center">
            <AppButton type="submit" :loading="loading" :disabled="loading">
              Create
            </AppButton>
            <AppButton color="error" @click="resetForm">
              Clear form
            </AppButton>
          </div>
        </UForm>
      </template>
    </UModal>
  </div>
</template>
