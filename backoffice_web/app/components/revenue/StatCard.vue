<script setup lang="ts">
defineProps<{
  label: string;
  value: string;
  trend: string;
  up: boolean;
  icon: string;
  desc: string;
}>();
</script>

<template>
  <div
    class="bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5 flex flex-col gap-4"
  >
    <div class="flex items-center justify-between">
      <span class="text-sm text-gray-500 dark:text-white/40 font-medium">{{
        label
      }}</span>
      <div
        class="w-8 h-8 rounded-xl bg-brand-50 dark:bg-brand/10 flex items-center justify-center"
      >
        <UIcon :name="icon" class="size-4 text-brand" />
      </div>
    </div>
    <div>
      <p class="text-3xl font-bold text-gray-900 dark:text-white">
        {{ value }}
      </p>
      <div class="flex items-center gap-1.5 mt-1.5">
        <UIcon
          :name="up ? 'heroicons:arrow-trending-up' : 'heroicons:arrow-trending-down'"
          :class="up ? 'text-brand' : 'text-red-500'"
          class="size-3.5"
        />
        <span
          :class="up ? 'text-brand' : 'text-red-500'"
          class="text-xs font-semibold"
        >
          {{ trend }}
        </span>
        <span class="text-xs text-gray-400 dark:text-white/25">{{ desc }}</span>
      </div>
    </div>
  </div>
</template>
