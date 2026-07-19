<script setup lang="ts">
import * as v from "valibot";
import type { FormSubmitEvent } from "@nuxt/ui";
import { useInvitationsStore } from "~/stores/invitations";

useHead({ title: "Invitations" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:paper-airplane",
    ariaLabel: "Invitations",
    title: "Invitations",
  },
});

const invitationsStore = useInvitationsStore();
const toast = useToast();

const loading = ref(true);
const sending = ref(false);

const schema = v.object({
  email: v.pipe(v.string(), v.email("Please enter a valid email address.")),
});

type Schema = v.InferOutput<typeof schema>;

const state = reactive<Schema>({ email: "" });

const invitations = computed(() => invitationsStore.invitations);
const pendingInvitations = computed(() =>
  invitations.value.filter((i) => i.status === "Pending"),
);
const hasInvitations = computed(() => invitations.value.length > 0);

onMounted(async () => {
  try {
    await invitationsStore.fetchAllInvitations();
  } finally {
    loading.value = false;
  }
});

async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  sending.value = true;
  try {
    await invitationsStore.createInvitation(data.email);
    toast.add({
      title: "Invitation sent",
      description: `An invitation has been sent to ${data.email}.`,
      color: "success",
    });
    state.email = "";
  } catch (err: any) {
    toast.add({
      title: "Error",
      description:
        err?.message || "Failed to send invitation. Please try again.",
      color: "error",
    });
  } finally {
    sending.value = false;
  }
}

async function onRevoke(identifier: string) {
  try {
    await invitationsStore.deleteInvitation(identifier);
    toast.add({ title: "Invitation revoked", color: "success" });
  } catch {
    toast.add({ title: "Failed to revoke invitation", color: "error" });
  }
}

async function onBlock(identifier: string) {
  try {
    await invitationsStore.blockInvitation(identifier);
    toast.add({ title: "Invitation blocked", color: "success" });
  } catch {
    toast.add({ title: "Failed to block invitation", color: "error" });
  }
}
</script>

<template>
  <div class="space-y-6">
    <div v-if="loading" class="flex items-center justify-center h-[60vh]">
      <p class="text-sm text-muted">Loading invitations…</p>
    </div>

    <template v-else>
      <div>
        <h1 class="text-2xl font-semibold">Invitations</h1>
        <p class="text-gray-400 mt-1 text-sm">
          Manage and track workspace invitations.
        </p>
      </div>

      <!-- Send invitation form -->
      <div
        class="border border-gray-100 dark:border-white/5 rounded-2xl p-5 max-w-lg"
      >
        <p class="font-semibold text-sm mb-4">Send an invitation</p>
        <UForm
          :schema="schema"
          :state="state"
          class="flex items-end gap-3"
          :on-submit="onSubmit"
        >
          <UFormField
            v-slot="{ error }"
            label="Email address"
            name="email"
            required
            class="flex-1"
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          >
            <UInput
              v-model="state.email"
              placeholder="colleague@example.com"
              :ui="{ base: 'py-3 px-4' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>
          <AppButton
            :loading="sending"
            :disabled="sending"
            type="submit"
            class="shrink-0 rounded px-5 py-3 text-sm text-white cursor-pointer"
          >
            Send
          </AppButton>
        </UForm>
      </div>

      <!-- Invitations list -->
      <div v-if="!hasInvitations" class="text-center py-12">
        <p class="text-sm text-muted">No invitations yet.</p>
      </div>

      <TeamsInvitationTable
        v-else
        :invitations="invitations"
        @revoke="onRevoke"
      />
    </template>
  </div>
</template>
