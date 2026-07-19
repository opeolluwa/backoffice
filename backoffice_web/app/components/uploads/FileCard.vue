<script setup lang="ts">
import type { UploadsInterface } from "~/bindings/UploadsInterface";

defineProps<{
  file: UploadsInterface;
  loaded: boolean;
}>();

defineEmits<{
  copy: [id: string];
  delete: [id: string];
}>();

function formatSize(bytes: number) {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
</script>

<template>
  <div
    class="group relative rounded-xl border border-default bg-white dark:bg-white/3 overflow-hidden flex flex-col"
  >
    <div
      class="aspect-square bg-gray-100 flex items-center justify-center overflow-hidden"
    >
      <template v-if="loaded">
        <NuxtImg
          provider="imagekit"
          :src="file.filePath"
          height="300"
        />
      </template>
      <div
        v-else
        class="w-full h-full animate-pulse bg-gray-200 dark:bg-white/10"
      />
    </div>

    <div class="flex gap-2 items-center justify-between">
      <div class="px-2.5 py-2 flex flex-col gap-0.5">
        <p class="text-xs font-medium truncate leading-tight">
          {{ file.name }}
        </p>
        <p class="text-[11px] text-muted">
          {{ formatSize(Number(file.fileSize) || 0) }}
        </p>
      </div>

      <button
        class="size-6 rounded-full bg-black/50 text-white flex items-center justify-center"
        aria-label="Copy file ID"
        @click.stop="$emit('copy', file.identifier)"
      >
        <UIcon name="heroicons:clipboard" class="size-3.5" />
      </button>
    </div>

    <div
      class="absolute top-1.5 right-1.5 flex gap-1 opacity-0 group-hover:opacity-100 transition-opacity"
    >
      <button
        class="size-6 rounded-full bg-black/50 text-white flex items-center justify-center"
        aria-label="Remove file"
        @click.stop="$emit('delete', file.identifier)"
      >
        <UIcon name="heroicons:trash" class="size-3.5 text-red-500" />
      </button>
    </div>
  </div>
</template>
