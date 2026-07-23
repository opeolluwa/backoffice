<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";
import { h, resolveComponent } from "vue";
import type { CustomersInterface } from "~/bindings/CustomersInterface";

defineProps<{
  customers: CustomersInterface[];
}>();

const emit = defineEmits<{
  delete: [id: string];
}>();

const UBadge = resolveComponent("UBadge");
const UButton = resolveComponent("UButton");
const UDropdownMenu = resolveComponent("UDropdownMenu");

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function getColumns(): TableColumn<CustomersInterface>[] {
  return [
    {
      accessorKey: "identifier",
      header: "ID",
      cell: ({ row }) =>
        h("span", { class: "font-mono text-xs text-muted" }, row.getValue("identifier")),
    },
    {
      accessorKey: "userIdentifier",
      header: "User",
      cell: ({ row }) =>
        h("span", { class: "font-medium text-sm" }, row.getValue("userIdentifier")),
    },
    {
      accessorKey: "createdAt",
      header: "Created",
      cell: ({ row }) => formatDate(row.getValue("createdAt")),
    },
    {
      accessorKey: "updatedAt",
      header: "Updated",
      cell: ({ row }) => {
        const val = row.getValue("updatedAt") as string | null;
        return val ? formatDate(val) : "—";
      },
    },
    {
      id: "actions",
      cell: ({ row }) => {
        const customer = row.original;
        return h(
          "div",
          { class: "text-right" },
          h(
            UDropdownMenu,
            {
              content: { align: "end" },
              items: [
                { type: "label", label: "Actions" },
                { type: "separator" },
                {
                  label: "Delete",
                  icon: "i-lucide-trash",
                  class: "text-red-500",
                  onSelect() {
                    emit("delete", customer.identifier);
                  },
                },
              ],
              "aria-label": "Customer actions",
            },
            () =>
              h(UButton, {
                icon: "i-lucide-ellipsis-vertical",
                color: "neutral",
                variant: "ghost",
                class: "ml-auto",
                "aria-label": "Customer actions",
              }),
          ),
        );
      },
    },
  ];
}
</script>

<template>
  <UTable
    :data="customers"
    :columns="getColumns()"
    class="rounded border border-default"
  />
</template>
