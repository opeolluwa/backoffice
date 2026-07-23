<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";
import { h, resolveComponent } from "vue";
import type { ComplaintsInterface } from "~/bindings/ComplaintsInterface";
import type { CustomersInterface } from "~/bindings/CustomersInterface";
import type { ComplaintStatus } from "~/bindings/ComplaintStatus";

defineProps<{
  complaints: ComplaintsInterface[];
  customers: CustomersInterface[];
}>();

const emit = defineEmits<{
  edit: [complaint: ComplaintsInterface];
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

const statusColors: Record<string, "info" | "warning" | "success" | "error"> = {
  Open: "info",
  InProgress: "warning",
  Resolved: "success",
  Closed: "error",
};

function getColumns(): TableColumn<ComplaintsInterface>[] {
  return [
    {
      accessorKey: "subject",
      header: "Subject",
      cell: ({ row }) =>
        h("div", { class: "flex flex-col" }, [
          h("span", { class: "font-medium text-sm" }, row.getValue("subject")),
          h(
            "span",
            { class: "text-xs text-muted truncate max-w-[300px]" },
            row.original.description,
          ),
        ]),
    },
    {
      accessorKey: "customerIdentifier",
      header: "Customer",
      cell: ({ row }) =>
        h("span", { class: "text-sm" }, row.getValue("customerIdentifier")),
    },
    {
      accessorKey: "status",
      header: "Status",
      cell: ({ row }) => {
        const status = row.getValue("status") as ComplaintStatus | null;
        const label = status || "Open";
        return h(
          UBadge,
          {
            color: statusColors[label] || "neutral",
            variant: "subtle",
            size: "sm",
          },
          () => label,
        );
      },
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
        const complaint = row.original;
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
                  label: "Edit",
                  icon: "i-lucide-pencil",
                  onSelect() {
                    emit("edit", complaint);
                  },
                },
                {
                  label: "Delete",
                  icon: "i-lucide-trash",
                  class: "text-red-500",
                  onSelect() {
                    emit("delete", complaint.identifier);
                  },
                },
              ],
              "aria-label": "Complaint actions",
            },
            () =>
              h(UButton, {
                icon: "i-lucide-ellipsis-vertical",
                color: "neutral",
                variant: "ghost",
                class: "ml-auto",
                "aria-label": "Complaint actions",
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
    :data="complaints"
    :columns="getColumns()"
    class="rounded border border-default"
  />
</template>
