<script setup lang="ts">
definePageMeta({
  layout: 'dashboard',
})
</script>

<template>
  <div class="flex flex-col justify-center items-center min-h-screen">
    <h1>You currently don&apos;t have any collection</h1>
    <NuxtLink to="/collections/create-collection"
        class="border-black/5 border-2 text-black px-3 py-4 rounded mt-3 cursor-pointer flex items-center justify-center gap-x-2"
    >
      <UIcon name="heroicons:plus-circle" class="size-5"/>
      Add collection
    </NuxtLink>
  </div>
</template>

<style scoped>

</style>