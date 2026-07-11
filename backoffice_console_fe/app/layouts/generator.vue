<template>
  <UDashboardGroup class="container">
    <UDashboardSidebar>
    <div class="flex items-center justify-center  gap-1 p-2">
        <h6>Backoffice Console</h6>
          <UColorModeButton />
    </div>
      <ul class="flex flex-col gap-1 p-2">
        <li v-for="link in links" :key="link.to">
          <NuxtLink
            :to="link.to"
            class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm hover:bg-accented transition-colors"
            active-class="bg-accented"
          >
            <UIcon :name="link.icon" class="w-5 h-5" />
            <span>{{ link.label }}</span>
          </NuxtLink>
        </li>
      </ul>
    </UDashboardSidebar>

    <slot />
  </UDashboardGroup>
</template>

<script lang="ts" setup>
const links = [
  {
    label: "Entity",
    icon: "lucide:database",
    to: "/entity",
  },
  {
    label: "Service",
    icon: "lucide:server",
    to: "/service",
  },
  {
    label: "Repository",
    icon: "lucide:git-branch",
    to: "/repository",
  },
  {
    label: "Handlers",
    icon: "lucide:pointer",
    to: "/handlers",
  },
  {
    label: "Modules",
    icon: "lucide:puzzle",
    to: "/modules",
  },
  {
    label: "Views",
    icon: "lucide:monitor",
    to: "/views",
  },
];
</script>
