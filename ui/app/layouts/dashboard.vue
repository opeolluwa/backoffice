<script setup lang="ts">
interface Route {
  label: string,
  icon: string,
  path: string
}

const routes: Route[] = [
  {
    label: "Home",
    path: "/home",
    icon: "heroicons:home-solid",
  },

];


import * as v from 'valibot'
import type {FormSubmitEvent} from '@nuxt/ui'

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

</script>

<template>
  <div class="grid grid-cols-12 h-screen">
    <nav class="col-span-2 border-r border-gray-200  relative px-3 flex flex-col">

      <div class="flex-1 overflow-y-auto">
        <!-- sidebar content here -->
<!--        <app-logo class="size-8"/>-->
        <ul class="space-y-2">
          <li v-for="route in routes" :key="route.path">
            <NuxtLink
                :to="route.path"
                class="flex items-center gap-2 px-3 py-4 rounded-lg text-gray-700 hover:bg-gray-100 transition-colors"
                active-class=""
            >
              <UIcon :name="route.icon" class="w-5 h-5"/>
              <span>{{ route.label }}</span>
            </NuxtLink>
          </li>
        </ul>
      </div>
      <UButton
          class="bg-black text-white px-3 py-4 rounded w-full mt-auto mb-3 cursor-pointer flex items-center justify-center gap-x-2"
      >

        <UIcon name="heroicons:arrow-left-end-on-rectangle-20-solid" class="size-5"/>
        Logout
      </UButton>
    </nav>

    <main class="col-span-10 h-screen overflow-y-scroll overflow-x-hidden">

      <!-- main content -->
      <div class=" flex justify-between py-3 px-6  border-b border-gray-200 hidden">
        <div>
          <UForm :schema="schema" :state="state">

          </UForm>
        </div>
        <div class="flex items-center gap-x-2 justify-center">

          <UIcon name="heroicons:bell" class="size-5"/>
          <UAvatar src="https://github.com/benjamincanac.png" class="rounded-full size-8 "/>
        </div>

      </div>

      <slot/>
    </main>
  </div>
</template>

<style scoped>

</style>
