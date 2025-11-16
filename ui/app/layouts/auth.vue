<script setup lang="ts">
const items = [
  {
    title: "Capturing Moments,",
    subtitle: "Creating Memories",
  },
  {
    title: "Photography that Inspires",
    subtitle: "Experience the Art",
  },
  {
    title: "Your Story,",
    subtitle: "Perfectly Told",
  },
];
</script>

<template>
  <div class="h-screen overflow-y-hidden w-full sm:grid grid-cols-12 gap-x-px">
    <div
      class="hidden sm:flex items-center justify-end flex-col pb-24 col-span-7 bg-[url('/bg.jpg')] bg-center bg-cover bg-black/50 dark:bg-dark/100 bg-blend-multiply relative rounded"
    >
      <UCarousel
        dots
        :autoplay="{ delay: 2500 }"
        loop
        :items="items"
        class="w-full max-w-md mx-auto"
        :ui="{ dot: 'w-8 h-1 rounded-full' }"
      >
        <template #item="{ item }">
          <div class="relative w-full h-80 rounded-2xl overflow-hidden">
        
            <div
              class="absolute bottom-10 left-0 right-0 text-center text-white px-4"
            >
              <h2 class="text-xl font-semibold">
                {{ item.title }}
              </h2>
              <p class="text-sm opacity-90 mt-1">
                {{ item.subtitle }}
              </p>
            </div>
          </div>
        </template>
      </UCarousel>
    </div>

    <div class="col-span-5 sm:flex flex-col justify-center w-[65%] mx-auto">
      <slot />
    </div>
  </div>
</template>

<style scoped></style>
