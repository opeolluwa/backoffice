<script setup lang="ts">
import * as v from 'valibot'
import type {FormSubmitEvent} from '@nuxt/ui'
import api from "~/plugin/api";

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

  const resp = await api.post("/users/login", {email: event.data.email, passpwrd: event.data.password});

  console.log(resp)
}

definePageMeta({
  layout: "auth"
})
</script>

<template>

  <h1 class="capitalize text-center text-5xl font-bold">Welcome</h1>
  <p class="text-center text-gray-400 leading-6 mt-2">Please enter your email and password</p>
  <UForm
      :schema="schema"
      :state="state"
      class="space-y-4 px-36 w-full mt-6"
      @submit="onSubmit"
  >
    <!-- Email Field -->
    <UFormField label="Email" name="email" required v-slot="{ error }" :ui="{
       error:'text-red-500 text-sm mt-1'
    }">
      <UInput
          v-model="state.email"
          :ui="{ base: 'py-4 px-6' , }"
          :class="[
      'border-2 rounded w-full transition-colors',
      error ? 'border-red-500 focus:border-red-500' : 'border-gray-300 focus:border-black'
    ]"
      />

    </UFormField>


    <!-- Password Field -->
    <UFormField label="Password" name="password" :error="false" required v-slot="{ error }" :ui="{
       error:'text-red-500 text-sm mt-1'
    }">
      <UInput
          v-model="state.password"
          type="password"
          :ui="{ base: 'py-4 px-6' }"
          :class="[
        'border-2 rounded w-full transition-colors',
        error ? 'border-red-500 focus:border-red-500' : 'border-gray-300 focus:border-black'
      ]"
      />
    </UFormField>

    <UButton
        type="submit"
        class="bg-black flex justify-center items-center text-center w-full rounded py-4 text-white cursor-pointer"
    >
      Login
    </UButton>
  </UForm>

</template>

<style scoped>

</style>