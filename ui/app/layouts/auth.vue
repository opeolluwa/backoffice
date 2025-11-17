<script setup lang="ts">
const items = [
  {
    title: "Powerful tools for your workflow",
    subtitle: "Automate. Organize. Ship.",
  },
  {
    title: "Manage your website content",
    subtitle: "Back Office gives you full control",
  },
  {
    title: "Collaborate with your team",
    subtitle: "Permissions, roles & fast publishing",
  },
];

</script>

<template>
  <div class="h-screen overflow-y-hidden w-full sm:grid grid-cols-12 gap-x-px">
    <div
      class="hidden sm:flex items-center justify-end flex-col pb-24 col-span-7 bg-[url('/bg.jpg')] bg-center bg-cover bg-black/50 dark:bg-dark/100 bg-blend-multiply relative rounded"
    >
    <NuxtImg src="/ijobaa.png"/>
      <UCarousel
        dots
        :autoplay="{ delay: 2500 }"
        loop
        :items="items"
        class="w-full max-w-md mx-auto"
        :ui="{ dot: 'w-8 h-1 rounded-full' }"
      >
        <template #default="{ item }">
          <div class="relative w-full h-80 rounded-2xl overflow-hidden">
            <div
              class="absolute bottom-10 left-0 right-0 text-center text-white px-4"
            >
              <h2 class="text-5xl  text-white/90 font-semibold">
                {{ item.title }}
              </h2>
              <p class="text-sm opacity-90 text-gray-400  mt-1">
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
