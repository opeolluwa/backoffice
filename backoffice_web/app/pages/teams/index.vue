<script setup lang="ts">
import { useTeamsStore } from "~/stores/teams";
import { useInvitationsStore } from "~/stores/invitations";

useHead({ title: "Team Members" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:users",
    ariaLabel: "Team",
    title: "Team members",
  },
});

const teamsStore = useTeamsStore();
const invitationsStore = useInvitationsStore();

const loading = ref(true);
const openInvite = ref(false);
const inviteLoading = ref(false);
const inviteDialog = ref<InstanceType<any>>();

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

const members = computed(() => teamsStore.members);
const invitations = computed(() => invitationsStore.invitations);
const pendingInvitations = computed(() =>
  invitations.value.filter((i) => i.status === "Pending"),
);
const hasMembers = computed(() => members.value.length > 0);
const hasContent = computed(
  () => hasMembers.value || pendingInvitations.value.length > 0,
);

const toast = useToast();

async function onMemberBlock(identifier: string) {
  await teamsStore.blockMember(identifier);
}

async function onMemberUnblock(identifier: string) {
  await teamsStore.unblockMember(identifier);
}

async function onMemberRemove(identifier: string) {
  await teamsStore.deleteMember(identifier);
}

async function onInvitationRevoke(identifier: string) {
  try {
    await invitationsStore.deleteInvitation(identifier);
    toast.add({ title: "Invitation revoked", color: "success" });
  } catch {
    toast.add({ title: "Failed to revoke invitation", color: "error" });
  }
}

async function onInviteSubmit(email: string) {
  inviteLoading.value = true;
  try {
    await invitationsStore.createInvitation(email);
    toast.add({
      title: "Invitation sent",
      description: `An invitation has been sent to ${email}.`,
      color: "success",
    });
    openInvite.value = false;
    inviteDialog.value?.reset();
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

onMounted(async () => {
  try {
    await Promise.all([
      teamsStore.fetchAllMembers(),
      invitationsStore.fetchAllInvitations(),
    ]);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="space-y-2">
    <div v-if="loading" class="flex items-center justify-center h-[60vh]">
      <p class="text-sm text-muted">Loading team members…</p>
    </div>

    <template v-else>
      <AppPageHeader
        v-if="hasContent"
        title="Team members"
        subtitle="Manage your team and invitations.È"
        cta-text="Add team member"
        @cta="openInvite = true"
      />

      <AppEmptyState
        v-if="!hasMembers && pendingInvitations.length === 0"
        title="No team members yet"
        description="Invite your first team member to get started."
        action-label="Add team member"
        @action="openInvite = true"
      />

      <template v-else>
        <TeamsInvitationTable
          v-if="pendingInvitations.length > 0"
          :invitations="pendingInvitations"
          @revoke="onInvitationRevoke"
        />

        <TeamsMemberTable
          :members="members"
          :role-groups="roleGroups"
          @block="onMemberBlock"
          @unblock="onMemberUnblock"
          @remove="onMemberRemove"
        />
      </template>

      <TeamsInviteDialog
        ref="inviteDialog"
        v-model:open="openInvite"
        :loading="inviteLoading"
        @submit="onInviteSubmit"
      />
    </template>
  </div>
</template>
