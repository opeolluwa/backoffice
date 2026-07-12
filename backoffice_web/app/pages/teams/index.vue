<script setup lang="ts">
import type { FormSubmitEvent, TableColumn } from "@nuxt/ui";
import * as v from "valibot";
import { h, resolveComponent } from "vue";
import type { Row } from "@tanstack/vue-table";
import type { TeamsInterface } from "~/bindings/TeamsInterface";

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:users",
    ariaLabel: "Team",
    title: "Team members",
  },
});

const mockTeamsInterfaces: TeamsInterface[] = [];

const roleGroups = [
  {
    role: "admin" as const,
    label: "Admin users",
    description:
      "Admins can add and remove users and manage organization-level settings.",
  },
  {
    role: "member" as const,
    label: "Account users",
    description:
      "Account users can assess and review risks, questionnaires, and identify breaches.",
  },
  {
    role: "viewer" as const,
    label: "Viewers",
    description: "Viewers have read-only access to the workspace.",
  },
];

const members = ref<TeamsInterface[]>(mockTeamsInterfaces);

const hasMembers = computed(() => members.value.length > 0);

function membersForRole(role: TeamsInterface["role"]) {
  return members.value.filter((m) => m.role === role);
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

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

function avatarColor(identifier: string) {
  const idx =
    identifier.charCodeAt(identifier.length - 1) % avatarColors.length;
  return avatarColors[idx];
}

// ─── Table Columns ────────────────────────────────────────────────────────────

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
      accessorKey: "dateAdded",
      header: "Date added",
      cell: ({ row }) => formatDate(row.getValue("dateAdded")),
    },
    {
      accessorKey: "lastActive",
      header: "Last active",
      cell: ({ row }) => formatDate(row.getValue("lastActive")),
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
      onSelect() {
        // TODO: navigate to member profile
      },
    },
    {
      label: member.blocked ? "Unblock" : "Block",
      icon: member.blocked ? "i-lucide-shield-check" : "i-lucide-shield-off",
      onSelect() {
        const idx = members.value.findIndex(
          (m) => m.identifier === member.identifier,
        );
        // if (idx !== -1) {
        //   members.value[idx].blocked = !members.value[idx].blocked;
        // }
      },
    },
    {
      label: "Remove",
      icon: "i-lucide-trash",
      class: "text-red-500",
      onSelect() {
        members.value = members.value.filter(
          (m) => m.identifier !== member.identifier,
        );
      },
    },
  ];
}

// ─── Invite Modal ─────────────────────────────────────────────────────────────

const openInvite = ref(false);

const inviteSchema = v.object({
  email: v.pipe(v.string(), v.email("Please enter a valid email address.")),
  role: v.pipe(v.string(), v.minLength(1, "Please select a role.")),
  name: v.pipe(v.string(), v.minLength(1, "Name is required.")),
});

type InviteSchema = v.InferOutput<typeof inviteSchema>;

const inviteState = reactive<InviteSchema>({ email: "", role: "", name: "" });
const inviteLoading = ref(false);
const toast = useToast();

const roleOptions = [
  { label: "Admin", value: "admin" },
  { label: "Member", value: "member" },
  { label: "Viewer", value: "viewer" },
];

function resetInviteForm() {
  inviteState.email = "";
  inviteState.role = "";
  inviteState.name = "";
}

async function onInviteSubmit({ data }: FormSubmitEvent<InviteSchema>) {
  inviteLoading.value = true;
  try {
    // TODO: replace with actual API call
    // await api.post("/invitations", data);

    // Optimistically add to mock list
    members.value.push({
      identifier: `01HZTEAM${Date.now()}`,
      name: data.name,
      email: data.email,
      role: data.role as TeamsInterface["role"],
      dateAdded: new Date().toISOString().slice(0, 10),
      lastActive: new Date().toISOString().slice(0, 10),
      blocked: false,
    });

    toast.add({
      title: "Invitation sent",
      description: `An invitation has been sent to ${data.email}.`,
      color: "success",
    });

    openInvite.value = false;
    resetInviteForm();
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to send invitation. Please try again.",
      color: "error",
    });
  } finally {
    inviteLoading.value = false;
  }
}

const columns = getColumns();
</script>

<template>
  <div class="space-y-2">
    <!-- Header: only shown when members exist -->
    <div v-if="hasMembers" class="flex items-start justify-between mb-8">
      <UButton class="px-4 py-2 shrink-0" @click="openInvite = true">
        Add team member
      </UButton>
    </div>

    <!-- Empty State -->
    <AppEmptyState
      v-if="!hasMembers"
      title="No team members yet"
      description="Invite your first team member to get started."
      action-label="Add team member"
      @action="openInvite = true"
    />

    <!-- Role Groups -->
    <template v-else>
      <div
        v-for="group in roleGroups"
        :key="group.role"
        class="grid grid-cols-1 lg:grid-cols-[260px_1fr] gap-6 py-6 border-b border-default last:border-b-0"
      >
        <!-- Group label -->
        <div class="space-y-1">
          <p class="font-semibold text-sm">{{ group.label }}</p>
          <p class="text-xs text-muted leading-relaxed">
            {{ group.description }}
          </p>
        </div>

        <!-- Members table or empty group -->
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
            :columns="columns"
            class="rounded border border-default"
          />
        </div>
      </div>
    </template>

    <!-- Invite Modal -->
    <UModal
      v-model:open="openInvite"
      title="Add team member"
      description="Send an invitation to join your workspace."
      close-icon="heroicons:x-mark"
    >
      <template #body>
        <UForm
          class="space-y-4"
          :schema="inviteSchema"
          :state="inviteState"
          :on-submit="onInviteSubmit"
        >
          <AppInput
            v-model="inviteState.name"
            label="Full name"
            name="name"
            placeholder="John Doe"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          />

          <AppInput
            v-model="inviteState.email"
            label="Email address"
            placeholder="colleague@example.com"
            name="email"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          />

          <AppSelect
            v-model="inviteState.role"
            label="Role"
            name="role"
            :items="roleOptions"
            placeholder="Please select a role"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          />

          <AppButton
            type="submit"
            :loading="inviteLoading"
            :disabled="inviteLoading"
          >
            Send invitation
          </AppButton>
        </UForm>
      </template>
    </UModal>
  </div>
</template>

<style scoped></style>
