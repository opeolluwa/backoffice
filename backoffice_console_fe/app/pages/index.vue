<template>
  <div class="flex flex-wrap gap-4 pt-8">
    <div class="w-1/4" v-for="component in conponents" :key="component.name">
      <UPageCard
        :title="component.name"
        :description="component.description"
        :icon="component.icon"
        class="px-3 py-2 hover:bg-accented"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
const conponents = ref([
  {
    name: "entity",
    description: "create entities",
    icon: "",
  },
  {
    name: "service",
    description: "create services",
    icon: "",
  },
  {
    name: "repository",
    description: "create repositories",
    icon: "",
  },
  {
    name: "handlers",
    description: "create handlers",
    icon: "",
  },
  {
    name: "modules",
    description: "create modules",
    icon: "",
  },
  {
    name: "views",
    description: "create views",
    icon: "",
  },
]);
</script>
