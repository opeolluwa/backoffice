<script setup lang="ts">
import { useCountryStore } from "~/stores/country";
import { useUploadStore } from "~/stores/uploads";
import { useCurrency } from "~/composables/useCurrency";
import { z } from "zod";

const open = defineModel<boolean>("open", { default: false });

defineProps<{
  loading: boolean;
}>();

const emit = defineEmits<{
  submit: [
    payload: {
      picture?: string;
      name: string;
      description: string;
      price: number;
      currencyIdentifier: string;
    },
  ];
}>();

const countryStore = useCountryStore();
const uploadStore = useUploadStore();
const { defaultCountry, ensureLoaded } = useCurrency();

type ImageSource = "upload" | "library";
const imageSource = ref<ImageSource>("upload");
const newFile = ref<File | null>(null);
const selectedUploadId = ref<string | null>(null);
const openLibrary = ref(false);

const imageUploads = computed(() =>
  uploadStore.uploads.filter((u) => u.fileType === "Image"),
);

const selectedUpload = computed(() =>
  selectedUploadId.value
    ? (uploadStore.uploads.find(
        (u) => u.identifier === selectedUploadId.value,
      ) ?? null)
    : null,
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
    avatar: c.flag ? c.flag : undefined,
    value: c.identifier,
  })),
);

function reset() {
  state.name = "";
  state.description = "";
  state.price = 0;
  state.currencyIdentifier = defaultCountry.value?.identifier ?? "";
  newFile.value = null;
  selectedUploadId.value = null;
  imageSource.value = "upload";
}

watch(
  open,
  (isOpen) => {
    if (isOpen) {
      ensureLoaded().then(() => {
        state.currencyIdentifier = defaultCountry.value?.identifier ?? "";
      });
    }
  },
  { immediate: true },
);

function pickFromLibrary() {
  openLibrary.value = true;
}

function confirmLibrary() {
  openLibrary.value = false;
  if (selectedUploadId.value) {
    imageSource.value = "library";
  }
}

function removeLibraryPick() {
  selectedUploadId.value = null;
  imageSource.value = "upload";
}

async function onSubmit() {
  let pictureId: string | undefined;

  if (imageSource.value === "upload" && newFile.value) {
    const created = await uploadStore.createUpload({
      file: newFile.value,
      name: newFile.value.name,
    });
    if (created && "identifier" in created) {
      pictureId = created.identifier;
    }
  } else if (imageSource.value === "library" && selectedUploadId.value) {
    pictureId = selectedUploadId.value;
  }

  emit("submit", {
    picture: pictureId,
    name: state.name,
    description: state.description,
    price: state.price,
    currencyIdentifier: state.currencyIdentifier,
  });
}

defineExpose({ reset });
</script>

<template>
  <UModal
    v-model:open="open"
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

          <div v-if="imageSource === 'upload'">
            <UFileUpload v-model="newFile" class="w-full min-h-48" />
            <button
              type="button"
              class="mt-2 text-xs text-gray-500 dark:text-white/40 hover:text-gray-700 dark:hover:text-white/60 cursor-pointer"
              @click="pickFromLibrary"
            >
              Or choose from library
            </button>
          </div>

          <div v-else-if="selectedUpload" class="space-y-2">
            <div
              class="relative w-32 h-32 rounded-xl overflow-hidden border border-gray-200 dark:border-white/10"
            >
              <img
                :src="selectedUpload.url"
                :alt="selectedUpload.name"
                class="w-full h-full object-cover"
              />
              <button
                type="button"
                class="absolute top-1 right-1 size-5 rounded-full bg-black/60 text-white flex items-center justify-center cursor-pointer"
                @click="removeLibraryPick"
              >
                <UIcon name="heroicons:x-mark" class="size-3" />
              </button>
            </div>
            <button
              type="button"
              class="text-xs text-gray-500 dark:text-white/40 hover:text-gray-700 dark:hover:text-white/60 cursor-pointer"
              @click="pickFromLibrary"
            >
              Change image
            </button>
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
          <AppButton color="error" @click="reset"> Clear form </AppButton>
          <AppButton type="submit" :loading="loading" :disabled="loading">
            Create
          </AppButton>
        </div>
      </UForm>
    </template>
  </UModal>

  <UModal
    v-model:open="openLibrary"
    title="Choose image"
    description="Select an image from your uploads"
    close-icon="heroicons:x-mark"
    size="xl"
  >
    <template #body>
      <AppLightBox v-model="selectedUploadId" :images="imageUploads" />
    </template>
    <template #footer>
      <div class="flex justify-end gap-2">
        <AppButton color="error" @click="openLibrary = false">
          Cancel
        </AppButton>
        <AppButton :disabled="!selectedUploadId" @click="confirmLibrary">
          Confirm
        </AppButton>
      </div>
    </template>
  </UModal>
</template>
