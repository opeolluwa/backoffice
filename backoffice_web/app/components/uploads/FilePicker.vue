<script setup lang="ts">
import type { CreateUploadRequest } from "~/bindings/CreateUploadRequest";

const open = defineModel<boolean>("open", { default: false });

defineProps<{
  loading: boolean;
}>();

const emit = defineEmits<{
  submit: [state: Partial<CreateUploadRequest>];
}>();

const state = reactive<Partial<CreateUploadRequest>>({
  file: null,
  name: "",
  file_type: null,
});

function onSubmit() {
  emit("submit", { ...state });
}

function reset() {
  state.file = null;
  state.name = "";
  state.file_type = null;
}

defineExpose({ reset });
</script>

<template>
  <UModal
    v-model:open="open"
    title="Upload files"
    description="Drag and drop files or click to browse."
    close-icon="heroicons:x-mark"
    :dismissible="loading != true && state.file != null && open == true"
  >
    <template #body>
      <UForm class="space-y-4" :state="state" @submit="onSubmit">
        <UFileUpload v-model="state.file" class="w-full min-h-48" />
        <AppInput
          v-model="state.name"
          label="File name"
          name="fileName"
          placeholder="example.jpg"
          required
          :ui="{ error: 'text-red-500 text-sm mt-1' }"
        />

        <AppButton
          type="submit"
          :size="'lg'"
          :loading="loading"
          :disabled="loading"
          @click="onSubmit"
        >
          Upload
        </AppButton>
      </UForm>
    </template>
  </UModal>
</template>
