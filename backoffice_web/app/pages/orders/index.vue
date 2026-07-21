<script setup lang="ts">
import type { FormSubmitEvent, TableColumn } from "@nuxt/ui";
import * as v from "valibot";
import api from "~/plugin/api";
import { useOrdersStore } from "~/stores/orders";
import { h, resolveComponent } from "vue";
import type { Row } from "@tanstack/vue-table";
import { getPaginationRowModel } from "@tanstack/vue-table";
import type { OrdersInterface } from "~/bindings/OrdersInterface";

useHead({ title: "Orders" });

const ordersStore = useOrdersStore();

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:puzzle-piece",
    ariaLabel: "Orders",
    title: "Orders",
  },
});

const UButton = resolveComponent("UButton");
const UDropdownMenu = resolveComponent("UDropdownMenu");

const toast = useToast();

const columns: TableColumn<OrdersInterface>[] = [
  {
    accessorKey: "name",
    header: "Name",
    cell: ({ row }) => `${row.getValue("name")}`,
  },
  {
    accessorKey: "description",
    header: "Description",
    cell: ({ row }) => `${row.getValue("description")}`,
  },
  {
    accessorKey: "createdAt",
    header: "Date created",
    cell: ({ row }) => {
      return new Date(row.getValue("createdAt")).toLocaleString("en-US", {
        day: "numeric",
        month: "short",
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      });
    },
  },
  {
    id: "actions",
    cell: ({ row }) => {
      return h(
        "div",
        { class: "text-right text-red-500" },
        h(
          UDropdownMenu,
          {
            content: {
              align: "end",
            },
            items: getRowItems(row),
            "aria-label": "Actions dropdown",
          },
          () =>
            h(UButton, {
              icon: "i-lucide-ellipsis-vertical",
              color: "neutral",
              variant: "ghost",
              class: "ml-auto",
              "aria-label": "Actions dropdown",
            }),
        ),
      );
    },
  },
];

function getRowItems(row: Row<OrdersInterface>) {
  const router = useRouter();
  const identifier = row.original.identifier;

  return [
    {
      type: "label",
      label: "Actions",
    },
    { type: "separator" },
    {
      label: "Update",
      icon: "i-lucide-pencil",
      onSelect() {
        router.push(`/orders/${identifier}/update`);
      },
    },
    {
      label: "Delete",
      icon: "i-lucide-trash",
      class: "text-red-500",
      async onSelect() {
        try {
          await api.delete(`/orders/${identifier}`);
          toast.add({
            title: "Deleted",
            description: "Orders deleted successfully.",
            color: "success",
          });
          await ordersStore.fetchOrders();
        } catch {
          toast.add({
            title: "Error",
            description: "Failed to delete orders.",
            color: "error",
          });
        }
      },
    },
  ];
}

const schema = v.object({
  name: v.pipe(v.string(), v.minLength(1, "Name is required")),
  description: v.pipe(v.string(), v.minLength(1, "Description is required")),
});

type Schema = v.InferOutput<typeof schema>;

const openForm = ref(false);
const state = reactive<Schema>({
  name: "",
  description: "",
});

const resetForm = () => {
  state.name = "";
  state.description = "";
};

const fetchingItems = ref(false);

const items = ref<OrdersInterface[]>();
const nullItems = computed(() => !items.value?.length);

const loading = ref(false);
async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    const res = await api.post("/orders", data);
    if (res.status !== 201) {
      throw new Error(res.data?.message || "Failed to create orders");
    }
    toast.add({
      title: "Success",
      description: "Orders created successfully.",
    });
    openForm.value = false;
    resetForm();
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to create orders. Please try again.",
      color: "error",
    });
  } finally {
    loading.value = false;
    await ordersStore.fetchOrders();
    items.value = ordersStore.orders;
  }
}

onMounted(async () => {
  try {
    await ordersStore.fetchOrders();
    items.value = ordersStore.orders;
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to load orders. Please try again.",
      color: "error",
    });
  } finally {
    fetchingItems.value = false;
  }
});

const pagination = ref({ pageIndex: 0, pageSize: 10 });
const search = ref("");

const filteredItems = computed(() => {
  const query = search.value.trim().toLowerCase();
  return (items.value || []).filter(() => {
    return !query;
  });
});

const table = useTemplateRef("table");
</script>

<template>
  <div>
    <PageLoader v-if="fetchingItems" />

    <AppEmptyState
      v-if="nullItems"
      icon="heroicons:puzzle-piece"
      title="No orders yet"
      description="Create your first orders to get started."
      action-label="Create first orders"
      @action="openForm = true"
    />

    <div v-else>
      <div
        class="flex flex-col lg:flex-row gap-3 mb-5 px-4 py-3 border rounded border-accented items-end"
      >
        <UInput
          v-model="search"
          class="max-w-sm"
          placeholder="Search by name / description"
        />
      </div>

      <UTable
        ref="table"
        v-model:pagination="pagination"
        :data="filteredItems"
        :loading="fetchingItems"
        loading-animation="carousel"
        :columns="columns"
        sticky="header"
        :pagination-options="{
          getPaginationRowModel: getPaginationRowModel(),
        }"
      />

      <div class="flex justify-center border-t border-default pt-4 mt-6">
        <UPagination
          :default-page="
            (table?.tableApi?.getState().pagination.pageIndex || 0) + 1
          "
          :items-per-page="table?.tableApi?.getState().pagination.pageSize"
          :total="table?.tableApi?.getFilteredRowModel().rows.length"
          @update:page="(p) => table?.tableApi?.setPageIndex(p - 1)"
        />
      </div>
    </div>

    <UModal
      v-model:open="openForm"
      title="Create Orders"
      description="A orders lets you manage your data"
      close-icon="heroicons:x-mark"
    >
      <template #body>
        <UForm
          class="space-y-4"
          :schema="schema"
          :state="state"
          :on-submit="onSubmit"
        >
          <div class="flex justify-between items-center">
            <UButton
              type="submit"
              class="dark:text-white/90 py-3 px-4"
              :loading="loading"
              :disabled="loading"
            >
              Continue
            </UButton>
            <UButton
              variant="subtle"
              color="muted"
              class="dark:text-white/90 py-3 px-4"
              @click="resetForm"
            >
              Clear form
            </UButton>
          </div>
        </UForm>
      </template>
    </UModal>

    <AppContentButton
      v-show="nullItems === false"
      class="fixed bottom-12 right-20"
      @click="openForm = true"
    />
  </div>
</template>
