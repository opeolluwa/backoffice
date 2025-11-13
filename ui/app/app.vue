<script setup lang="ts">
import { useNuxtApp } from "#app";

const { $viewport } = useNuxtApp();

watch($viewport.breakpoint, (newBreakpoint, oldBreakpoint) => {
  console.log("Breakpoint updated:", oldBreakpoint, "->", newBreakpoint);
});

definePageMeta({
  colorMode: "light",
});
</script>

<template>
  <div>
    <div
      v-if="$viewport.isLessThan('tablet')"
      class="h-screen w-full flex flex-col justify-center items-center px-4 text-center"
    >
      <NuxtImg
        alt="unsupported media viewport"
        src="/error-icon.png"
        class="w-[100px]"
      />
      <h1 class="text-2xl mt-4 font-medium">Unsupported media viewport!</h1>
      <p class="text-gray-500 leading-5 mt-1 text-sm">
        Unfortunately, you cannot access this page on a
        {{ $viewport.breakpoint }}device Please use a tablet or desktop
      </p>
    </div>
    <NuxtLayout v-else>
      <NuxtPage />
    </NuxtLayout>
  </div>
</template>
