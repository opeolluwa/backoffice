<script setup lang="ts">
</script>

<template>
  <div
    class="h-screen w-full flex items-center justify-center overflow-hidden relative bg-linear-to-br from-brand-50 via-white to-brand-50 dark:bg-brand-dark-700"
  >
    <!-- Background image (dark only) -->
    <img
      src="/bg-dark.jpg"
      alt=""
      class="absolute inset-0 w-full h-full object-cover object-center hidden dark:block"
      aria-hidden="true"
    />

    <!-- Dark overlay (dark only) -->
    <div class="absolute inset-0 bg-brand-dark-700/75 hidden dark:block" />

    <!-- Subtle grid texture (dark only) -->
    <div
      class="absolute inset-0 opacity-[0.04] hidden dark:block"
      style="background-image: linear-gradient(rgba(255,255,255,.6) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.6) 1px, transparent 1px); background-size: 40px 40px;"
    />

    <!-- Subtle dot pattern (light only) -->
    <div
      class="absolute inset-0 opacity-40 dark:hidden"
      style="background-image: radial-gradient(circle, #00b87a22 1px, transparent 1px); background-size: 32px 32px;"
    />

    <!-- Glowing accent blobs -->
    <div
      class="absolute -top-32 -left-32 w-[480px] h-[480px] rounded-full bg-brand/10 blur-[120px] pointer-events-none dark:bg-brand/20"
    />
    <div
      class="absolute bottom-0 right-0 w-[320px] h-80 rounded-full bg-brand/8 blur-[100px] pointer-events-none dark:bg-brand/10"
    />

    <!-- Centered card -->
    <div
      class="relative z-10 w-1/3 min-w-80 mx-auto px-4 overflow-y-auto max-h-screen py-10"
    >
      <div
        class="bg-white border border-gray-100 rounded-2xl px-8 py-10 shadow-sm dark:bg-brand-dark-600/80 dark:backdrop-blur-md dark:border-white/8"
      >


        <!-- Page content -->
        <slot />
      </div>

      <!-- Footer -->
      <p class="text-center text-[11px] text-gray-400 mt-6 dark:text-white/20 ">
      Powered by <a href="https://github.com/opeolluwa/backoffice" target="_blank" class="underline font-medium">Backoffice</a> 
      </p>
    </div>
  </div>
</template>

<style scoped></style>
