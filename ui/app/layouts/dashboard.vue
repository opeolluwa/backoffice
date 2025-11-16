<script setup lang="ts">
import * as v from "valibot";
import type { FormSubmitEvent } from "@nuxt/ui";
import useLogout from "~/composables/useLogout";

interface Route {
  label: string;
  icon: string;
  path: string;
}

interface Divider {
  divider: boolean;
}

type RouteItem = Route | Divider;

const routes: RouteItem[] = [
  {
    label: "Home",
    path: "/home",
    icon: "heroicons:home",
  },

  {
    label: "Tasks",
    path: "/tasks",
    icon: "lucide:layout-list",
  },

  {
    label: "Metrics",
    path: "/metrics",
    icon: "heroicons:chart-bar-square",
  },

  {
    label: "Calendar",
    path: "/calendar",
    icon: "heroicons:calendar-date-range",
  },

  {
    divider: true,
  },
  {
    label: "Marketplace",
    path: "/marketplace",
    icon: "heroicons:building-storefront",
  },
  {
    label: "Collection",
    path: "/collections",
    icon: "heroicons:folder",
  },
  {
    divider: true,
  },
  {
    label: "Team",
    path: "/teams",
    icon: "heroicons:users",
  },
  {
    label: "Settings",
    path: "/settings",
    icon: "heroicons:cog",
  },
];

const schema = v.object({
  query: v.pipe(v.string()),
});

type Schema = v.InferOutput<typeof schema>;

const state = reactive({
  query: "",
});

const toast = useToast();

// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function onSubmit(event: FormSubmitEvent<Schema>) {
  toast.add({
    title: "Success",
    description: "The form has been submitted.",
    color: "success",
  });
  console.log(event.data);
}

const logout = async () => useLogout();
const getKey = (item: RouteItem) =>
  "divider" in item ? `div-${Math.random()}` : item.path;
</script>

<template>
  <div class="grid grid-cols-12 h-screen">
    <nav
      class="col-span-2 border-r border-gray-200 dark:border-gray-200/10 relative flex flex-col px-4 pt-12 bg-brand-50/20 dark:bg-brand-dark-400/5"
    >
      <div class="flex-1 overflow-y-auto">
        <ul class="space-y-2">
          <li v-for="item in routes" :key="getKey(item)">
            <!-- Divider -->
            <template v-if="'divider' in item">
              <div
                class="border-t border-gray-200 dark:border-gray-200/10 my-6"
              />
            </template>

            <!-- Normal route -->
            <template v-else>
              <NuxtLink
                :href="item.path"
                class="flex items-center hover:text-brand dark:hover:text-white/50 gap-2 px-3 my-2 py-2 rounded text-gray-500 transition-colors border-brand-400"
                active-class="bg-brand-50/90 dark:bg-brand-100/20 t"
              >
                <UIcon :name="item.icon" class="size-5" />
                <span>{{ item.label }}</span>
              </NuxtLink>
            </template>
          </li>
        </ul>
      </div>

      <UButton
        class="text-white px-3 py-3 rounded w-full mt-auto mb-3 cursor-pointer flex items-center justify-center gap-x-2 dark:bg-brand-600"
        @click="logout"
      >
        <UIcon
          name="heroicons:arrow-left-end-on-rectangle-20-solid"
          class="size-5"
        />
        Logout
      </UButton>
    </nav>

    <main class="col-span-10 h-screen overflow-y-scroll overflow-x-hidden">
      <!-- main content -->
      <header
        class="flex justify-between items-center px-8 border-b border-gray-200 bg-brand-50/20 dark:bg-brand-dark dark:border-gray-200/10 min-h-20 py-4"
      >
        <div>
          <UForm :schema="schema" :state="state">
            <UFormField
              v-slot="{ error }"
              name="name"
              required
              :ui="{
                error: 'text-red-500 text-sm mt-1',
              }"
            >
              <UInput
                placeholder="Search"
                icon="heroicons:magnifying-glass"
                :ui="{ base: 'py-2.5 px-6' }"
                variant="outline"
                :class="[
                  'w-full transition-colors rounded-3xl',
                  error
                    ? 'border-red-500 focus:border-red-500'
                    : 'border-gray-300 focus:border-black',
                ]"
              />
            </UFormField>
          </UForm>
        </div>
        <div class="flex items-center gap-x-5 justify-center">
          <UIcon name="heroicons:bell" class="_icon" />
          <UColorModeButton />
          <NuxtLink to="/account">
            <UserCard />
          </NuxtLink>
        </div>
      </header>

      <div class="px-8">
        <slot />
      </div>
    </main>
  </div>
</template>

<style scoped></style>
