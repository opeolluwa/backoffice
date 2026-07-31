<script setup lang="ts">
defineProps<{
  label: string;
  value: string;
  icon: string;
  desc: string;
}>();
</script>

<template>
  <div
    class="bg-white dark:bg-gray-800 border border-gray-100 dark:border-white/5 rounded-2xl p-5 flex flex-col gap-4"
  >
    <div class="flex items-center justify-between">
      <span class="text-sm text-gray-500 dark:text-white/40 font-medium">{{
        label
      }}</span>
      <div
        class="w-8 h-8 rounded-xl bg-gray-50 dark:bg-white/10 flex items-center justify-center"
      >
        <UIcon :name="icon" class="size-4 text-brand" />
      </div>
    </div>
    <div>
      <p class="text-3xl font-bold text-gray-900 dark:text-white">
        {{ value }}
      </p>
    </div>
  </div>
</template>
