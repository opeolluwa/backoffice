<template>
<div class="flex flex-wrap gap-4">

    <div class="" v-for="component in conponents" :key="component.name"></div>
</div>
</template>


<script lang="ts" setup>
const conponents = ref([
  {
    name: "entity",
    description: "create entities",
    icon: ""
  }
])
</script>