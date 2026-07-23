<script setup lang="ts">
import { z } from "zod";
import type { ComplaintsInterface } from "~/bindings/ComplaintsInterface";
import type { ComplaintStatus } from "~/bindings/ComplaintStatus";

const open = defineModel<boolean>("open", { default: false });

const props = defineProps<{
  complaint: ComplaintsInterface | null;
  loading: boolean;
}>();

const emit = defineEmits<{
  submit: [
    id: string,
    payload: {
      subject?: string;
      description?: string;
      status?: string;
    },
  ];
}>();

const state = reactive({
  subject: "",
  description: "",
  status: "" as string,
});

const schema = z.object({
  subject: z.string().min(1, "Subject is required"),
  description: z.string().min(1, "Description is required"),
  status: z.string().min(1, "Status is required"),
});

const statusOptions = [
  { label: "Open", value: "Open" },
  { label: "In Progress", value: "InProgress" },
  { label: "Resolved", value: "Resolved" },
  { label: "Closed", value: "Closed" },
];

watch(
  () => props.complaint,
  (c) => {
    if (c) {
      state.subject = c.subject;
      state.description = c.description;
      state.status = c.status || "Open";
    }
  },
  { immediate: true },
);

function reset() {
  if (props.complaint) {
    state.subject = props.complaint.subject;
    state.description = props.complaint.description;
    state.status = props.complaint.status || "Open";
  }
}

function onSubmit() {
  if (!props.complaint) return;
  emit("submit", props.complaint.identifier, {
    subject: state.subject,
    description: state.description,
    status: state.status,
  });
}

defineExpose({ reset });
</script>

<template>
  <UModal
    v-model:open="open"
    title="Update complaint"
    description="Edit complaint details or change status"
    close-icon="heroicons:x-mark"
  >
    <template #body>
      <UForm
        class="space-y-4"
        :schema="schema"
        :state="state"
        :on-submit="onSubmit"
      >
        <AppInput
          v-model="state.subject"
          label="Subject"
          name="subject"
          placeholder="Brief summary"
        />

        <AppInput
          v-model="state.description"
          label="Description"
          name="description"
          placeholder="Detailed description"
        />

        <AppSelect
          v-model="state.status"
          label="Status"
          name="status"
          :items="statusOptions"
          placeholder="Select status"
          required
        />

        <div class="flex justify-end gap-2">
          <AppButton color="error" @click="open = false">Cancel</AppButton>
          <AppButton type="submit" :loading="loading" :disabled="loading">
            Save
          </AppButton>
        </div>
      </UForm>
    </template>
  </UModal>
</template>
