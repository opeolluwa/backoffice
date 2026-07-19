<script setup lang="ts">
import type { UploadsInterface } from "~/bindings/UploadsInterface";

const props = defineProps<{
  files: UploadsInterface[];
  loadedPreviews: Set<string>;
}>();

defineEmits<{
  copy: [id: string];
  delete: [id: string];
}>();

function onTileVisible(id: string) {
  props.loadedPreviews.add(id);
}

function createTileRef(id: string) {
  return (el: Element | null) => {
    if (!el) return;
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry?.isIntersecting) {
          onTileVisible(id);
          observer.disconnect();
        }
      },
      { threshold: 0.1 },
    );
    observer.observe(el);
  };
}
</script>

<template>
  <div class="space-y-3">
    <p class="text-sm text-muted">
      {{ files.length }} {{ files.length === 1 ? "file" : "files" }}
    </p>

    <div
      class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3"
    >
      <div
        v-for="file in files"
        :key="file.identifier"
        :ref="createTileRef(file.identifier)"
      >
        <UploadsFileCard
          :file="file"
          :loaded="loadedPreviews.has(file.identifier)"
          @copy="(id) => $emit('copy', id)"
          @delete="(id) => $emit('delete', id)"
        />
      </div>
    </div>
  </div>
</template>
