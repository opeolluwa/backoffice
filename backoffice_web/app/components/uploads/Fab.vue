<template>
  <button
    class="fixed bottom-6 right-6 size-12 rounded-full cursor-pointer bg-primary text-white flex items-center justify-center shadow-lg hover:bg-primary-500 transition-colors z-50"
    aria-label="Upload file"
    @click="$emit('click')"
  >
    <UIcon name="heroicons:plus" class="size-5" />
  </button>
</template>
