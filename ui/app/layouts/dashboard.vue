<script setup lang="ts">
import * as v from 'valibot'
import type {FormSubmitEvent} from '@nuxt/ui'
import useLogout from "~/composibles/useLogout";

interface Route {
  label: string,
  icon: string,
  path: string
}

const routes: Route[] | { divider: string } = [
  {
    label: "Home",
    path: "/home",
    icon: "heroicons:home",
  },

  {
    label: "Users",
    path: "/users",
    icon: "heroicons:users"
  },
  // {
  //   label: "Account",
  //   path: "/account",
  //   icon: "heroicons:user"
  // },

  {
    label: "Marketplace",
    path: "/marketplace",
    icon: "heroicons:building-storefront"
  },
  {
    label: "Collection",
    path: "/collections",
    icon: "heroicons:folder"
  },
  {
    label: "Settings",
    path: "/settings",
    icon: "heroicons:cog"
  },
];



const schema = v.object({
  query: v.pipe(v.string()),
})

type Schema = v.InferOutput<typeof schema>

const state = reactive({
  query: '',
})

const toast = useToast()

async function onSubmit(event: FormSubmitEvent<Schema>) {
  toast.add({title: 'Success', description: 'The form has been submitted.', color: 'success'})
  console.log(event.data)
}

const logout = async () => useLogout();
</script>

<template>
  <div class="grid grid-cols-12 h-screen">
    <nav class="col-span-2 border-r border-gray-200  relative flex flex-col px-4 pt-12  bg-brand-50/10">

      <div class="flex-1 overflow-y-auto">
        <ul class="space-y-2">
          <li v-for="route in routes" :key="route.path">
            <NuxtLink
                :to="route.path"
                class="flex items-center   hover:text-brand gap-2 px-4 py-4 rounded text-gray-500 transition-colors"
                active-class="bg-brand-50/90 text-brand"
            >
              <UIcon :name="route.icon" class="size-5"/>
              <span>{{ route.label }}</span>
            </NuxtLink>
          </li>
        </ul>
      </div>
      <UButton
          @click="logout"
          class=" text-white px-3 py-4 rounded w-full mt-auto mb-3 cursor-pointer flex items-center justify-center gap-x-2"
      >

        <UIcon name="heroicons:arrow-left-end-on-rectangle-20-solid" class="size-5"/>
        Logout
      </UButton>
    </nav>

    <main class="col-span-10 h-screen overflow-y-scroll overflow-x-hidden">

      <!-- main content -->
      <div class=" flex justify-between py-3 px-8 border-b border-gray-200">
        <div>
          <UForm :schema="schema" :state="state">

          </UForm>
        </div>
        <div class="flex items-center gap-x-2 justify-center">

          <UIcon name="heroicons:bell" class="size-5"/>
          <UAvatar src="https://github.com/benjamincanac.png" />
        </div>

      </div>

      <div class="px-8">
        <slot/>
      </div>
    </main>
  </div>
</template>

<style scoped>

</style>
