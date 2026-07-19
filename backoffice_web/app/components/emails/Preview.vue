<script setup lang="ts">
import type { EmailsInterface } from "@bindings/EmailsInterface";

defineProps<{
  email: EmailsInterface | null;
}>();

defineEmits<{
  toggleStar: [email: EmailsInterface, event: MouseEvent];
}>();

function formatFullDate(dateStr: string) {
  return new Date(dateStr).toLocaleString("en-US", {
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });
}
</script>

<template>
  <div
    class="flex-1 bg-white dark:bg-brand-dark-600 rounded-xl border border-gray-100 dark:border-white/5 overflow-hidden flex flex-col"
  >
    <div
      v-if="!email"
      class="flex-1 flex flex-col items-center justify-center gap-4 text-center px-8"
    >
      <div
        class="w-16 h-16 rounded-2xl bg-gray-50 dark:bg-white/5 flex items-center justify-center"
      >
        <UIcon
          name="heroicons:envelope"
          class="size-8 text-gray-300 dark:text-white/20"
        />
      </div>
      <div>
        <p class="font-medium text-base text-gray-700 dark:text-white/50">
          No message selected
        </p>
        <p class="text-sm text-muted mt-1">
          Select a message from the list to preview it here.
        </p>
      </div>
    </div>

    <template v-else>
      <div class="px-8 py-5 border-b border-gray-100 dark:border-white/5">
        <div class="flex items-start justify-between gap-4">
          <div class="min-w-0">
            <h2
              class="text-base font-semibold text-gray-900 dark:text-white leading-snug"
            >
              {{ email.subject }}
            </h2>
            <p class="text-xs text-gray-400 dark:text-white/30 mt-1">
              {{ formatFullDate(email.dateSent) }}
            </p>
          </div>

          <div class="flex items-center gap-2 shrink-0">
            <span
              v-if="email.hasAttachments"
              class="flex items-center gap-1 text-xs text-gray-400 dark:text-white/30"
            >
              <UIcon name="heroicons:paper-clip" class="size-3.5" />
              Attachments
            </span>
            <span
              v-if="email.tag"
              class="text-xs bg-brand-50 dark:bg-brand/10 text-brand dark:text-brand-300 rounded-full px-2.5 py-0.5 font-medium"
            >
              {{ email.tag }}
            </span>
            <button
              class="transition-colors cursor-pointer"
              :class="
                email.isStarred
                  ? 'text-amber-400'
                  : 'text-gray-300 dark:text-white/20 hover:text-amber-300'
              "
              @click="$emit('toggleStar', email, $event)"
            >
              <UIcon
                :name="
                  email.isStarred ? 'heroicons:star-solid' : 'heroicons:star'
                "
                class="size-4"
              />
            </button>
          </div>
        </div>

        <div class="mt-4 space-y-1.5">
          <div class="flex items-center gap-2 text-xs">
            <span class="text-gray-400 dark:text-white/25 w-8">From</span>
            <span class="font-medium text-gray-700 dark:text-white/70">
              {{ email.senderEmail }}
            </span>
          </div>
          <div class="flex items-center gap-2 text-xs">
            <span class="text-gray-400 dark:text-white/25 w-8">To</span>
            <span class="font-medium text-gray-700 dark:text-white/70">
              {{ email.recipientEmail }}
            </span>
          </div>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto px-8 py-6">
        <div
          v-if="email.body.includes('<')"
          class="prose prose-sm dark:prose-invert max-w-none text-gray-700 dark:text-white/70"
          v-html="email.body"
        />
        <pre
          v-else
          class="whitespace-pre-wrap text-sm text-gray-700 dark:text-white/70 font-sans leading-relaxed"
        >{{ email.body }}</pre>
      </div>
    </template>
  </div>
</template>
