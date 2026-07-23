<script setup lang="ts">
import { z } from "zod";
import { useCustomersStore } from "~/stores/customers";

const open = defineModel<boolean>("open", { default: false });

defineProps<{
  loading: boolean;
}>();

const emit = defineEmits<{
  submit: [
    payload: {
      customerIdentifier: string;
      orderIdentifier?: string;
      subject: string;
      description: string;
    },
  ];
}>();

const customersStore = useCustomersStore();

const state = reactive({
  customerIdentifier: "",
  orderIdentifier: "",
  subject: "",
  description: "",
});

const schema = z.object({
  customerIdentifier: z.string().min(1, "Customer is required"),
  subject: z.string().min(1, "Subject is required"),
  description: z.string().min(1, "Description is required"),
});

const customerOptions = computed(() =>
  customersStore.customers.map((c) => ({
    label: c.userIdentifier,
    value: c.identifier,
  })),
);

function reset() {
  state.customerIdentifier = "";
  state.orderIdentifier = "";
  state.subject = "";
  state.description = "";
}

function onSubmit() {
  emit("submit", {
    customerIdentifier: state.customerIdentifier,
    orderIdentifier: state.orderIdentifier || undefined,
    subject: state.subject,
    description: state.description,
  });
}

defineExpose({ reset });
</script>

<template>
  <UModal
    v-model:open="open"
    title="Create complaint"
    description="File a new customer complaint"
    close-icon="heroicons:x-mark"
  >
    <template #body>
      <UForm
        class="space-y-4"
        :schema="schema"
        :state="state"
        :on-submit="onSubmit"
      >
        <AppSelect
          v-model="state.customerIdentifier"
          label="Customer"
          name="customerIdentifier"
          :items="customerOptions"
          placeholder="Select customer"
          required
        />

        <AppInput
          v-model="state.orderIdentifier"
          label="Order ID (optional)"
          name="orderIdentifier"
          placeholder="Link to an order"
        />

        <AppInput
          v-model="state.subject"
          label="Subject"
          name="subject"
          placeholder="Brief summary of the complaint"
        />

        <AppInput
          v-model="state.description"
          label="Description"
          name="description"
          placeholder="Detailed description"
        />

        <div class="flex justify-between items-center">
          <AppButton color="error" @click="reset">Clear form</AppButton>
          <AppButton type="submit" :loading="loading" :disabled="loading">
            Create
          </AppButton>
        </div>
      </UForm>
    </template>
  </UModal>
</template>
