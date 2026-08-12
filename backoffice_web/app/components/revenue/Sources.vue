<script setup lang="ts">
interface Source {
  label: string;
  pct: number;
  color: string;
}

defineProps<{
  sources: Source[];
}>();
</script>

<template>
  <div
    class=" border border-gray-100 dark:border-white/5 rounded-2xl p-5"
  >
    <p class="text-sm font-semibold text-gray-700 dark:text-white/80 mb-1">
      By source
    </p>
    <p class="text-xs text-gray-400 dark:text-white/30 mb-5">
      Revenue breakdown
    </p>

    <div class="space-y-4">
      <div v-for="source in sources" :key="source.label">
        <div class="flex items-center justify-between mb-1.5">
          <div class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full shrink-0" :class="source.color" />
            <span class="text-xs text-gray-600 dark:text-white/50">{{
              source.label
            }}</span>
          </div>
          <span class="text-xs font-medium text-gray-700 dark:text-white/60"
            >{{ source.pct }}%</span
          >
        </div>
        <div
          class="h-1.5 rounded-full bg-gray-100 dark:bg-white/5 overflow-hidden"
        >
          <div
            class="h-full rounded-full transition-all"
            :class="source.color"
            :style="{ width: `${source.pct}%` }"
          />
        </div>
      </div>
    </div>

    <div
      v-if="sources.every((s) => s.pct === 0)"
      class="mt-6 flex flex-col items-center gap-2 text-center"
    >
      <div
        class="w-10 h-10 rounded-xl bg-gray-50 dark:bg-white/5 flex items-center justify-center"
      >
        <UIcon
          name="heroicons:chart-bar"
          class="size-5 text-gray-300 dark:text-white/20"
        />
      </div>
      <p class="text-xs text-gray-400 dark:text-white/25">
        No revenue data yet.
      </p>
    </div>
  </div>
</template>
