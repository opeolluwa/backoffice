<script setup lang="ts"></script>

<template>
  <div class="h-screen overflow-y-hidden w-full sm:grid grid-cols-12 gap-x-px">
    <div
      class="hidden sm:block col-span-7 bg-[url('/bg.jpg')] bg-center bg-cover bg-black/50 dark:bg-dark/100 bg-blend-multiply relative rounded"
    />

    <div class="col-span-5 sm:flex flex-col justify-center w-[65%] mx-auto">
      <slot />
    </div>
  </div>
</template>

<style scoped></style>
