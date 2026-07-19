<script setup lang="ts">
import type { EmailsInterface } from "@bindings/EmailsInterface";
import { useEmailStore } from "~/stores/emails";

useHead({ title: "Messages" });

definePageMeta({
  layout: "dashboard",
  breadcrumb: {
    icon: "heroicons:envelope",
    ariaLabel: "Messages",
    title: "Messages",
  },
});

const emailStore = useEmailStore();
const toast = useToast();

const loading = ref(false);
const search = ref("");
const activeTab = ref<"all" | "unread" | "starred">("all");
const selectedEmail = ref<EmailsInterface | null>(null);

onMounted(async () => {
  loading.value = true;
  try {
    await emailStore.fetchEmails();
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to load messages.",
      color: "error",
    });
  } finally {
    loading.value = false;
  }
});

const filteredEmails = computed(() => {
  let list = emailStore.emails;

  if (activeTab.value === "unread") list = list.filter((e) => !e.isRead);
  else if (activeTab.value === "starred")
    list = list.filter((e) => e.isStarred);

  const q = search.value.trim().toLowerCase();
  if (!q) return list;

  return list.filter(
    (e) =>
      e.subject.toLowerCase().includes(q) ||
      e.senderEmail.toLowerCase().includes(q) ||
      e.recipientEmail.toLowerCase().includes(q) ||
      (e.tag ?? "").toLowerCase().includes(q),
  );
});

const unreadCount = computed(
  () => emailStore.emails.filter((e) => !e.isRead).length,
);

async function selectEmail(email: EmailsInterface) {
  selectedEmail.value = email;
  if (!email.isRead) {
    await emailStore.markAsRead(email.identifier);
  }
}

async function toggleStar(email: EmailsInterface, event: MouseEvent) {
  event.stopPropagation();
  await emailStore.toggleStarred(email.identifier, !email.isStarred);
}
</script>

<template>
  <div class="flex gap-6 h-[calc(100vh-180px)] min-h-[500px]">
    <EmailsList
      :emails="filteredEmails"
      :loading="loading"
      :search="search"
      :active-tab="activeTab"
      :unread-count="unreadCount"
      :selected-id="selectedEmail?.identifier ?? null"
      @update:search="search = $event"
      @update:active-tab="activeTab = $event"
      @select="selectEmail"
      @toggle-star="toggleStar"
    />

    <EmailsPreview :email="selectedEmail" @toggle-star="toggleStar" />
  </div>
</template>
