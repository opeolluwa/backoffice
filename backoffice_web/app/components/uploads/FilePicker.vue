<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";

const open = defineModel<boolean>("open", { default: false });

defineProps<{
  loading: boolean;
}>();

const emit = defineEmits<{
  submit: [state: v.InferOutput<typeof schema>];
}>();

const schema = v.object({
  file: v.pipe(v.instance(File, "Please select a file.")),
  name: v.pipe(v.string(), v.minLength(1, "File name is required.")),
  file_type: v.nullable(v.string()),
});

type Schema = v.InferOutput<typeof schema>;

const state = reactive<Schema>({
  file: null as unknown as File,
  name: "",
  file_type: null,
});

function onSubmit({ data }: FormSubmitEvent<Schema>) {
  emit("submit", { ...data });
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
      <UForm
        class="space-y-4"
        :schema="schema"
        :state="state"
        @submit="onSubmit"
      >
        <UFileUpload v-model="state.file" class="w-full min-h-48" />
        <AppInput
          v-model="state.name"
          label="File name"
          name="name"
          placeholder="example.jpg"
        />

        <AppButton
          type="submit"
          :size="'lg'"
          :loading="loading"
          :disabled="loading"
        >
          Upload
        </AppButton>
      </UForm>
    </template>
  </UModal>
</template>
