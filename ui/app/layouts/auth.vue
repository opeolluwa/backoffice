<script setup lang="ts">

</script>


<template>
  <div class="h-screen overflow-y-hidden w-full sm:grid grid-cols-12 gap-x-px">
    <div class="hidden sm:block col-span-6 bg-[url('/bg-7.jpg')] bg-center bg-cover bg-black/20 bg-blend-multiply relative rounded"/>

    <div class="col-span-6 sm:flex flex-col justify-center items-center">
      <slot></slot>

    </div>


  </div>
</template>


<style scoped>

</style>