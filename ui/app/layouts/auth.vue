<script setup lang="ts">

</script>


<template>
  <div class="h-screen overflow-y-hidden w-full grid grid-cols-12 gap-x-px">
    <div class="col-span-6 bg-[url('/bg-2.jpg')] bg-center bg-cover bg-black/60 bg-blend-multiply relative"/>

    <div class="col-span-6 flex flex-col justify-center items-center">
      <slot></slot>
    </div>


  </div>
</template>


<style scoped>

</style>