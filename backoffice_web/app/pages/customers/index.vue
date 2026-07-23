<script setup lang="ts">
import { useCustomersStore } from "~/stores/customers";

useHead({ title: "Customers" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:users",
    ariaLabel: "Customers",
    title: "Customers",
  },
});

const customersStore = useCustomersStore();
const toast = useToast();

const isFetching = ref(true);
const hasCustomers = computed(() => customersStore.customers.length > 0);

async function onDeleteCustomer(identifier: string) {
  try {
    await customersStore.deleteCustomer(identifier);
    toast.add({ title: "Customer deleted", color: "success" });
  } catch {
    toast.add({ title: "Failed to delete customer", color: "error" });
  }
}

onMounted(async () => {
  try {
    await customersStore.fetchCustomers();
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
      v-else-if="!hasCustomers"
      icon="heroicons:users"
      title="No customers yet"
      description="Customers will appear here once they register."
    />

    <template v-else>
      <AppPageHeader
        title="Customers"
        subtitle="View and manage your customer base"
      />

      <CustomersCustomerList
        :customers="customersStore.customers"
        @delete="onDeleteCustomer"
      />
    </template>
  </div>
</template>
