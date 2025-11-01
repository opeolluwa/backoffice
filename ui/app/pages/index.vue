<script setup lang="ts">
import * as v from 'valibot'
import type {FormSubmitEvent} from '@nuxt/ui'

const schema = v.object({
  email: v.pipe(v.string(), v.email('Please enter a valid email address.')),
  password: v.pipe(v.string(), v.minLength(8, 'Must be at least 8 characters'))
})

type Schema = v.InferOutput<typeof schema>

const state = reactive({
  email: '',
  password: ''
})

const toast = useToast()

async function onSubmit(event: FormSubmitEvent<Schema>) {
  toast.add({title: 'Success', description: 'The form has been submitted.', color: 'success'})
  console.log(event.data)
}
</script>

<template>
  <div class="h-screen overflow-y-hidden w-full grid grid-cols-12 gap-x-px">
    <div class="col-span-6 bg-[url('/bg-2.jpg')] bg-center bg-cover bg-black/60 bg-blend-multiply relative"/>

    <div class="col-span-6 flex flex-col justify-center items-center">

      <h1 class="capitalize text-center text-5xl font-bold">Welcome</h1>
      <p class="text-center text-gray-400 leading-6 mt-2">Please enter your email and password</p>

      <UForm :schema="schema" :state="state" class="space-y-4 px-36 w-full mt-6" @submit="onSubmit">
        <UFormField label="Email" name="email"  required>
          <UInput v-model="state.email" :ui="{
 base:'py-4 px-6'
          }" class="border-2 rounded w-full "/>
        </UFormField>

        <UFormField label="Password" name="password" required>
          <UInput v-model="state.password" type="password" :ui="{
 base:'py-4 px-6'
          }" class=" border-2 rounded w-full"/>
        </UFormField>

        <UButton type="submit" class="bg-black flex justify-center items-center text-center w-full rounded py-4 text-white cursor-pointer">
Login
        </UButton>
      </UForm>
    </div>


  </div>
</template>

<style scoped>

</style>