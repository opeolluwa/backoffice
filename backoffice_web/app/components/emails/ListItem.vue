<script setup lang="ts">
import type { EmailsInterface } from "@bindings/EmailsInterface";

defineProps<{
  email: EmailsInterface;
  selected: boolean;
}>();

defineEmits<{
  select: [email: EmailsInterface];
  toggleStar: [email: EmailsInterface, event: MouseEvent];
}>();

function formatDate(dateStr: string) {
  const date = new Date(dateStr);
  const now = new Date();
  const isToday = date.toDateString() === now.toDateString();
  if (isToday) {
    return date.toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    });
  }
  return date.toLocaleDateString("en-US", { day: "numeric", month: "short" });
}
</script>

<template>
  <button
    class="w-full text-left px-4 py-3 border-b border-gray-50 dark:border-white/[0.03] hover:bg-gray-50 dark:hover:bg-white/[0.03] transition-colors cursor-pointer"
    :class="selected ? 'bg-gray-50/60 dark:bg-white/5' : ''"
    @click="$emit('select', email)"
  >
    <div class="flex items-start justify-between gap-2">
      <div class="flex items-center gap-2 min-w-0">
        <span
          class="shrink-0 size-1.5 rounded-full mt-1.5"
          :class="!email.isRead ? 'bg-gray-900 dark:bg-gray-100' : 'bg-transparent'"
        />
        <div class="min-w-0">
          <p
            class="text-xs truncate"
            :class="
              !email.isRead
                ? 'font-semibold text-gray-900 dark:text-white'
                : 'font-medium text-gray-600 dark:text-white/50'
            "
          >
            {{ email.senderEmail }}
          </p>
          <p
            class="text-xs truncate mt-0.5"
            :class="
              !email.isRead
                ? 'text-gray-800 dark:text-white/80'
                : 'text-gray-500 dark:text-white/35'
            "
          >
            {{ email.subject }}
          </p>
          <p
            class="text-[10px] text-gray-400 dark:text-white/20 truncate mt-0.5"
          >
            To: {{ email.recipientEmail }}
          </p>
        </div>
      </div>

      <div class="flex flex-col items-end gap-1.5 shrink-0">
        <span
          class="text-[10px] text-gray-400 dark:text-white/25 whitespace-nowrap"
        >
          {{ formatDate(email.dateSent) }}
        </span>
        <button
          class="transition-colors cursor-pointer"
          :class="
            email.isStarred
              ? 'text-amber-400'
              : 'text-gray-200 dark:text-white/10 hover:text-amber-300'
          "
          @click="$emit('toggleStar', email, $event)"
        >
          <UIcon
            :name="email.isStarred ? 'heroicons:star-solid' : 'heroicons:star'"
            class="size-3.5"
          />
        </button>
        <span
          v-if="email.tag"
          class="text-[9px] bg-gray-100 dark:bg-white/5 text-gray-500 dark:text-white/30 rounded px-1.5 py-0.5 uppercase tracking-wide"
        >
          {{ email.tag }}
        </span>
      </div>
    </div>
  </button>
</template>
