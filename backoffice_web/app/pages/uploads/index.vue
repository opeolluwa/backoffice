<script setup lang="ts">
import type { CreateUploadRequest } from "~/bindings/CreateUploadRequest";
import { useUploadStore } from "~/stores/uploads";

useHead({ title: "Uploads" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    ariaLabel: "Uploads",
    title: "Uploads",
  },
});

const uploadStore = useUploadStore();
const openUploadDialog = ref(false);
const isUploading = ref(false);
const isFetchingFiles = ref(true);
const hasFiles = computed(() => uploadStore?.uploads?.length > 0);
const files = computed(() => uploadStore.uploads.filter(Boolean));
const loadedPreviews = ref<Set<string>>(new Set());

const toast = useToast();
const confirmDelete = useConfirmDelete();
const uploadDialog = ref<InstanceType<any>>();

async function removeFile(identifier: string) {
  const confirmed = await confirmDelete({
    title: "Delete file?",
    description: "This action cannot be undone.",
  });

  if (!confirmed) return;

  await uploadStore.deleteOneUpload(identifier);
}

async function copyFileId(identifier: string) {
  try {
    await navigator.clipboard.writeText(identifier);
    toast.add({ title: "Copied file ID", color: "success" });
  } catch {
    toast.add({ title: "Failed to copy", color: "error" });
  }
}

async function onUploadSubmit(state: Partial<CreateUploadRequest>) {
  isUploading.value = true;

  try {
    await uploadStore.createUpload(state);
    uploadDialog.value?.reset();
    openUploadDialog.value = false;
    toast.add({ title: "File uploaded successfully", color: "success" });
  } catch {
    toast.add({ title: "Failed to upload file", color: "error" });
  } finally {
    isUploading.value = false;
  }
}

onMounted(async () => {
  try {
    await uploadStore.findAllUploads();
  } catch (error) {
    console.error(error);
  } finally {
    isFetchingFiles.value = false;
  }
});
</script>

<template>
  <div class="space-y-6">
    <AppEmptyState
      v-if="!hasFiles && !isFetchingFiles"
      icon="heroicons:arrow-up-tray"
      title="No files uploaded yet"
      description="Drag and drop files above or click to browse."
      action-label="Browse files"
      @action="openUploadDialog = true"
    />

    <template v-else-if="hasFiles">
      <AppPageHeader
        title="Uploads"
        subtitle="Manage your uploaded files"
        cta-text="Upload file"
        @cta="openUploadDialog = true"
      />

      <UploadsFileGrid
        È
        :files="files"
        :loaded-previews="loadedPreviews"
        @copy="copyFileId"
        @delete="removeFile"
      />
    </template>

    <UploadsFilePicker
      ref="uploadDialog"
      v-model:open="openUploadDialog"
      :loading="isUploading"
      @submit="onUploadSubmit"
    />

    <!-- <UploadsFab v-if="hasFiles" @click="openUploadDialog = true" /> -->
  </div>
</template>
