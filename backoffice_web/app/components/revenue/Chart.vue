<script setup lang="ts">
defineProps<{
  months: string[];
  values: number[];
  range: string;
}>();

function barHeight(val: number, max: number) {
  return Math.max(Math.round((val / max) * 100), 4);
}
</script>

<template>
  <div
    class="xl:col-span-2 bg-white dark:bg-brand-dark-600 border border-gray-100 dark:border-white/5 rounded-2xl p-5"
  >
    <div class="flex items-center justify-between mb-6">
      <div>
        <p class="text-sm font-semibold text-gray-700 dark:text-white/80">
          Revenue over time
        </p>
        <p class="text-xs text-gray-400 dark:text-white/30 mt-0.5">
          {{ range }}
        </p>
      </div>
    </div>

    <div class="flex items-end gap-3 h-36">
      <template v-for="(val, i) in values" :key="months[i]">
        <div class="flex-1 flex flex-col items-center gap-1.5">
          <div
            class="w-full rounded-md transition-all"
            :class="
              i === values.length - 1
                ? 'bg-brand'
                : 'bg-brand-100 dark:bg-brand/20'
            "
            :style="{ height: `${barHeight(val, Math.max(...values, 1))}%` }"
          />
          <span class="text-[9px] text-gray-400 dark:text-white/25 font-medium">
            {{ months[i] }}
          </span>
        </div>
      </template>
    </div>

    <div
      class="mt-4 pt-4 border-t border-gray-100 dark:border-white/5 flex items-center justify-between"
    >
      <div>
        <p class="text-lg font-bold text-gray-900 dark:text-white">
          ₦{{ values[values.length - 1]?.toLocaleString() ?? 0 }}
        </p>
        <p class="text-xs text-gray-400 dark:text-white/30">this month</p>
      </div>
    </div>
  </div>
</template>
