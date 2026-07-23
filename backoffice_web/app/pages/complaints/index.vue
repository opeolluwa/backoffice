<script setup lang="ts">
import { useComplaintsStore } from "~/stores/complaints";
import { useCustomersStore } from "~/stores/customers";
import type { ComplaintsInterface } from "~/bindings/ComplaintsInterface";

useHead({ title: "Complaints" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:exclamation-triangle",
    ariaLabel: "Complaints",
    title: "Complaints",
  },
});

const complaintsStore = useComplaintsStore();
const customersStore = useCustomersStore();
const toast = useToast();

const isFetching = ref(true);
const openCreate = ref(false);
const createLoading = ref(false);
const createDialog = ref<InstanceType<any>>();

const openUpdate = ref(false);
const updateLoading = ref(false);
const updateDialog = ref<InstanceType<any>>();
const selectedComplaint = ref<ComplaintsInterface | null>(null);

const hasComplaints = computed(() => complaintsStore.complaints.length > 0);

async function onCreateSubmit(payload: {
  customerIdentifier: string;
  orderIdentifier?: string;
  subject: string;
  description: string;
}) {
  createLoading.value = true;
  try {
    await complaintsStore.createComplaint(payload);
    createDialog.value?.reset();
    openCreate.value = false;
    toast.add({ title: "Complaint created", color: "success" });
  } catch (error: any) {
    toast.add({
      title: "Failed to create complaint",
      description: error?.message || "Please try again.",
      color: "error",
    });
  } finally {
    createLoading.value = false;
  }
}

function onEdit(complaint: ComplaintsInterface) {
  selectedComplaint.value = complaint;
  openUpdate.value = true;
}

async function onUpdateSubmit(
  id: string,
  payload: { subject?: string; description?: string; status?: string },
) {
  updateLoading.value = true;
  try {
    await complaintsStore.updateComplaint(id, payload);
    updateDialog.value?.reset();
    openUpdate.value = false;
    selectedComplaint.value = null;
    toast.add({ title: "Complaint updated", color: "success" });
  } catch (error: any) {
    toast.add({
      title: "Failed to update complaint",
      description: error?.message || "Please try again.",
      color: "error",
    });
  } finally {
    updateLoading.value = false;
  }
}

async function onDeleteComplaint(identifier: string) {
  try {
    await complaintsStore.deleteComplaint(identifier);
    toast.add({ title: "Complaint deleted", color: "success" });
  } catch {
    toast.add({ title: "Failed to delete complaint", color: "error" });
  }
}

onMounted(async () => {
  try {
    await Promise.all([
      complaintsStore.fetchComplaints(),
      customersStore.fetchCustomers(),
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
      v-else-if="!hasComplaints"
      icon="heroicons:exclamation-triangle"
      title="No complaints yet"
      description="Customer complaints will appear here."
      action-label="File a complaint"
      @action="openCreate = true"
    />

    <template v-else>
      <AppPageHeader
        title="Complaints"
        subtitle="Track and resolve customer issues"
        cta-text="File complaint"
        @cta="openCreate = true"
      />

      <ComplaintsComplaintList
        :complaints="complaintsStore.complaintsList"
        :customers="customersStore.customers"
        @edit="onEdit"
        @delete="onDeleteComplaint"
      />
    </template>

    <ComplaintsCreateComplaintDialog
      ref="createDialog"
      v-model:open="openCreate"
      :loading="createLoading"
      @submit="onCreateSubmit"
    />

    <ComplaintsUpdateComplaintDialog
      ref="updateDialog"
      v-model:open="openUpdate"
      :complaint="selectedComplaint"
      :loading="updateLoading"
      @submit="onUpdateSubmit"
    />
  </div>
</template>
