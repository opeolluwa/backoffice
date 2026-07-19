<script setup lang="ts">
import type { EmailsInterface } from "@bindings/EmailsInterface";

defineProps<{
  emails: EmailsInterface[];
  loading: boolean;
  search: string;
  activeTab: "all" | "unread" | "starred";
  unreadCount: number;
  selectedId: string | null;
}>();

defineEmits<{
  "update:search": [value: string];
  "update:activeTab": [value: "all" | "unread" | "starred"];
  select: [email: EmailsInterface];
  toggleStar: [email: EmailsInterface, event: MouseEvent];
}>();
</script>

<template>
  <div
    class="w-80 shrink-0 flex flex-col bg-white dark:bg-brand-dark-600 rounded-xl border border-gray-100 dark:border-white/5 overflow-hidden"
  >
    <div
      class="px-4 pt-4 pb-3 border-b border-gray-100 dark:border-white/5 space-y-3"
    >
      <UInput
        :model-value="search"
        placeholder="Search messages..."
        icon="heroicons:magnifying-glass"
        size="sm"
        @update:model-value="$emit('update:search', $event)"
      />

      <div class="flex gap-1">
        <button
          v-for="tab in ['all', 'unread', 'starred'] as const"
          :key="tab"
          class="flex-1 py-1.5 text-xs font-medium rounded-lg capitalize transition-colors cursor-pointer"
          :class="
            activeTab === tab
              ? 'bg-brand-50 dark:bg-brand/10 text-brand dark:text-brand-300'
              : 'text-gray-400 dark:text-white/30 hover:text-gray-700 dark:hover:text-white/60'
          "
          @click="$emit('update:activeTab', tab)"
        >
          {{ tab }}
          <span
            v-if="tab === 'unread' && unreadCount > 0"
            class="ml-1 bg-brand text-white text-[10px] rounded-full px-1.5 py-0.5"
          >
            {{ unreadCount }}
          </span>
        </button>
      </div>
    </div>

    <div class="flex-1 overflow-y-auto">
      <PageLoader v-if="loading" />

      <div
        v-else-if="emails.length === 0"
        class="flex flex-col items-center justify-center h-full gap-4 text-center px-6"
      >
        <div
          class="w-14 h-14 rounded-2xl bg-gray-50 dark:bg-white/5 flex items-center justify-center"
        >
          <UIcon
            name="heroicons:envelope-open"
            class="size-7 text-gray-300 dark:text-white/20"
          />
        </div>
        <div>
          <p class="font-medium text-sm text-gray-700 dark:text-white/60">
            No messages found
          </p>
          <p class="text-xs text-muted mt-1">
            Try adjusting your search or filter.
          </p>
        </div>
      </div>

      <EmailsListItem
        v-for="email in emails"
        :key="email.identifier"
        :email="email"
        :selected="selectedId === email.identifier"
        @select="$emit('select', email)"
        @toggle-star="(e, ev) => $emit('toggleStar', e, ev)"
      />
    </div>
  </div>
</template>
