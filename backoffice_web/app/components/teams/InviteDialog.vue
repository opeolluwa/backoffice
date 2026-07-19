<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import * as v from "valibot";

const open = defineModel<boolean>("open", { default: false });

defineProps<{
  loading: boolean;
}>();

const emit = defineEmits<{
  submit: [email: string];
}>();

const schema = v.object({
  email: v.pipe(v.string(), v.email("Please enter a valid email address.")),
  role: v.pipe(v.string(), v.minLength(1, "Please select a role.")),
  name: v.pipe(v.string(), v.minLength(1, "Name is required.")),
});

type Schema = v.InferOutput<typeof schema>;

const state = reactive<Schema>({ email: "", role: "", name: "" });

const roleOptions = [
  { label: "Admin", value: "admin" },
  { label: "Member", value: "member" },
  { label: "Viewer", value: "viewer" },
];

function reset() {
  state.email = "";
  state.role = "";
  state.name = "";
}

async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  emit("submit", data.email);
}

defineExpose({ reset });
</script>

<template>
  <UModal
    v-model:open="open"
    title="Add team member"
    description="Send an invitation to join your workspace."
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
          v-model="state.name"
          label="Full name"
          name="name"
          placeholder="John Doe"
          required
          :ui="{ error: 'text-red-500 text-sm mt-1' }"
        />

        <AppInput
          v-model="state.email"
          label="Email address"
          placeholder="colleague@example.com"
          name="email"
          required
          :ui="{ error: 'text-red-500 text-sm mt-1' }"
        />

        <AppSelect
          v-model="state.role"
          label="Role"
          name="role"
          :items="roleOptions"
          placeholder="Please select a role"
          required
          :ui="{ error: 'text-red-500 text-sm mt-1' }"
        />

        <AppButton type="submit" :loading="loading" :disabled="loading">
          Send invitation
        </AppButton>
      </UForm>
    </template>
  </UModal>
</template>
