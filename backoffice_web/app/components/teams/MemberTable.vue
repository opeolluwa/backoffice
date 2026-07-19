<script setup lang="ts">
import type { TableColumn } from "@nuxt/ui";
import { h, resolveComponent } from "vue";
import type { Row } from "@tanstack/vue-table";
import type { TeamsInterface } from "~/bindings/TeamsInterface";

const props = defineProps<{
  members: TeamsInterface[];
  roleGroups: { role: string; label: string; description: string }[];
}>();

const emit = defineEmits<{
  block: [id: string];
  unblock: [id: string];
  remove: [id: string];
}>();

const avatarColors = [
  "bg-violet-500",
  "bg-blue-500",
  "bg-emerald-500",
  "bg-orange-500",
  "bg-rose-500",
  "bg-cyan-500",
  "bg-amber-500",
  "bg-indigo-500",
];

function getInitials(name: string) {
  return name
    .split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
}

function avatarColor(identifier: string) {
  const idx =
    identifier.charCodeAt(identifier.length - 1) % avatarColors.length;
  return avatarColors[idx];
}

function formatDate(dateStr: string) {
  return new Date(dateStr).toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: "numeric",
  });
}

const UButton = resolveComponent("UButton");
const UDropdownMenu = resolveComponent("UDropdownMenu");
const UBadge = resolveComponent("UBadge");

function getColumns(): TableColumn<TeamsInterface>[] {
  return [
    {
      accessorKey: "name",
      header: "Name",
      cell: ({ row }) => {
        const member = row.original;
        return h("div", { class: "flex items-center gap-3" }, [
          h(
            "div",
            {
              class: `size-8 rounded-full flex items-center justify-center text-white text-xs font-semibold shrink-0 ${avatarColor(member.identifier)}`,
            },
            getInitials(member.name),
          ),
          h("div", { class: "flex flex-col" }, [
            h("span", { class: "font-medium text-sm" }, member.name),
            h("span", { class: "text-xs text-muted" }, member.email),
          ]),
        ]);
      },
    },
    {
      accessorKey: "createdAt",
      header: "Date added",
      cell: ({ row }) => formatDate(row.getValue("createdAt")),
    },
    {
      accessorKey: "updatedAt",
      header: "Last active",
      cell: ({ row }) => {
        const val = row.getValue("updatedAt") as string | null;
        return val ? formatDate(val) : "—";
      },
    },
    {
      accessorKey: "blocked",
      header: "Status",
      cell: ({ row }) => {
        const blocked = row.getValue("blocked") as boolean;
        return h(
          UBadge,
          {
            color: blocked ? "error" : "success",
            variant: "subtle",
            size: "sm",
          },
          () => (blocked ? "Blocked" : "Active"),
        );
      },
    },
    {
      id: "actions",
      cell: ({ row }) => {
        return h(
          "div",
          { class: "text-right" },
          h(
            UDropdownMenu,
            {
              content: { align: "end" },
              items: getRowItems(row),
              "aria-label": "Member actions",
            },
            () =>
              h(UButton, {
                icon: "i-lucide-ellipsis-vertical",
                color: "neutral",
                variant: "ghost",
                class: "ml-auto",
                "aria-label": "Member actions",
              }),
          ),
        );
      },
    },
  ];
}

function getRowItems(row: Row<TeamsInterface>) {
  const member = row.original;

  return [
    { type: "label", label: "Actions" },
    { type: "separator" },
    {
      label: "View profile",
      icon: "i-lucide-user",
      onSelect() {},
    },
    {
      label: member.blocked ? "Unblock" : "Block",
      icon: member.blocked ? "i-lucide-shield-check" : "i-lucide-shield-off",
      onSelect() {
        if (member.blocked) {
          emit("unblock", member.identifier);
        } else {
          emit("block", member.identifier);
        }
      },
    },
    {
      label: "Remove",
      icon: "i-lucide-trash",
      class: "text-red-500",
      onSelect() {
        emit("remove", member.identifier);
      },
    },
  ];
}

function membersForRole(role: string) {
  return props.members.filter((m) => m.role === role);
}
</script>

<template>
  <div
    v-for="group in roleGroups"
    :key="group.role"
    class="grid grid-cols-1 lg:grid-cols-[260px_1fr] gap-6 py-6 border-b border-default last:border-b-0"
  >
    <div class="space-y-1">
      <p class="font-semibold text-sm">{{ group.label }}</p>
      <p class="text-xs text-muted leading-relaxed">
        {{ group.description }}
      </p>
    </div>

    <div>
      <div
        v-if="membersForRole(group.role).length === 0"
        class="flex items-center gap-3 text-sm text-muted py-4 px-3 rounded border border-dashed border-default"
      >
        <UIcon name="i-lucide-users" class="size-4 shrink-0" />
        <span>No {{ group.label.toLowerCase() }} yet.</span>
      </div>

      <UTable
        v-else
        :data="membersForRole(group.role)"
        :columns="getColumns()"
        class="rounded border border-default"
      />
    </div>
  </div>
</template>
