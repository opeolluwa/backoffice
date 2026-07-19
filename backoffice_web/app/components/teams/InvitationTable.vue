<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";
import { h, resolveComponent } from "vue";
import type { InvitationInterface } from "~/bindings/InvitationInterface";

const props = defineProps<{
  invitations: InvitationInterface[];
}>();

const emit = defineEmits<{
  revoke: [id: string];
}>();

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

function getInitials(name: string) {
  return name
    .split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
}

const UButton = resolveComponent("UButton");
const UBadge = resolveComponent("UBadge");

function getColumns(): TableColumn<InvitationInterface>[] {
  return [
    {
      accessorKey: "email",
      header: "Email",
      cell: ({ row }) => {
        const inv = row.original;
        return h("div", { class: "flex items-center gap-3" }, [
          h(
            "div",
            {
              class:
                "size-8 rounded-full flex items-center justify-center bg-gray-200 dark:bg-white/10 text-xs font-semibold shrink-0",
            },
            getInitials(inv.email.split("@")[0]),
          ),
          h("div", { class: "flex flex-col" }, [
            h("span", { class: "font-medium text-sm" }, inv.email),
          ]),
        ]);
      },
    },
    {
      accessorKey: "status",
      header: "Status",
      cell: ({ row }) => {
        const status = row.getValue("status") as string | null;
        const color =
          status === "Pending"
            ? "warning"
            : status === "Accepted"
              ? "success"
              : status === "Rejected"
                ? "error"
                : "neutral";
        return h(
          UBadge,
          { color, variant: "subtle", size: "sm" },
          () => status ?? "Unknown",
        );
      },
    },
    {
      accessorKey: "createdAt",
      header: "Sent",
      cell: ({ row }) => formatDate(row.getValue("createdAt")),
    },
    {
      id: "actions",
      cell: ({ row }) => {
        const inv = row.original;
        return h("div", { class: "text-right" }, [
          h(UButton, {
            icon: "i-lucide-trash",
            color: "error",
            variant: "ghost",
            size: "sm",
            "aria-label": "Revoke invitation",
            onClick: () => emit("revoke", inv.identifier),
          }),
        ]);
      },
    },
  ];
}
</script>

<template>
  <div class="mb-8">
    <div class="mb-4">
      <p class="font-semibold text-sm">Pending invitations</p>
      <p class="text-xs text-muted mt-1">
        Invitations that have been sent but not yet accepted.
      </p>
    </div>
    <UTable
      :data="invitations"
      :columns="getColumns()"
      class="rounded border border-default"
    />
  </div>
</template>
