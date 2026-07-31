<script setup lang="ts"></script>

<template>
  <div
    class="h-screen w-full flex items-center justify-center overflow-hidden relative bg-app-200/90"
  >
    <div class="relative z-10 min-w-1/3 mx-auto px-4 overflow-y-auto py-10">
      <div
        class="border-none bg-white border-gray-100 rounded-2xl px-8 py-10 shadow-sm dark:border-white/8"
      >
        <div class="flex justify-center mb-6">
          <AppLogo size="md" />
        </div>
        <slot />
      </div>

      <!-- Footer -->
      <p class="text-center text-[11px] text-gray-400 mt-6 dark:text-white/20">
        Powered by
        <a
          href="https://github.com/opeolluwa/backoffice"
          target="_blank"
          class="underline font-medium"
          >Backoffice</a
        >
      </p>
    </div>
  </div>
</template>

<style scoped></style>
