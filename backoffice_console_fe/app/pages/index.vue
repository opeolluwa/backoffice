<template>
  <div class="flex flex-wrap gap-4 pt-8">
    <div class="w-1/4 cursor-pointer" v-for="component in conponents" :key="component.name">
      <UPageCard
        :title="component.name"
        :description="component.description"
        :icon="component.icon"
        class="px-3 py-2 hover:bg-accented cursor-pointer"
        :to="`/${component.name}`"
        :ui="{}"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
const conponents = ref([
  {
    name: "entity",
    description: "create entities",
    icon: "lucide:database",
  },
  {
    name: "service",
    description: "create services",
    icon: "lucide:server",
  },
  {
    name: "repository",
    description: "create repositories",
    icon: "lucide:git-branch",
  },
  {
    name: "handlers",
    description: "create handlers",
    icon: "lucide:pointer",
  },
  {
    name: "modules",
    description: "create modules",
    icon: "lucide:puzzle",
  },
  {
    name: "views",
    description: "create views",
    icon: "lucide:monitor",
  },
]);
</script>
