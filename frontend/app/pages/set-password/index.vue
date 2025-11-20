<script setup lang="ts">
definePageMeta({
  layout: "security",
});

const password = ref("");
const confirmPassword = ref("");
const formError = ref<string>();
const loading = ref<boolean>(false);

// eslint-disable-next-line @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any
async function onSubmit(values: any) {
  try {
    loading.value = true;
    formError.value = "";
    if (password.value !== confirmPassword.value) {
      formError.value = "Passwords do not match";
      return;
    }
  } catch (error) {
    console.log(error);
    formError.value = "Something went wrong";
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="flex flex-col items-center justify-center">
    <AppLeadingText>Set New Password</AppLeadingText>
    <p class="mb-2 text-gray-400 mt-1">Enter and confirm your new password</p>

    <UForm
      :state="{ password: '', confirmPassword: '' }"
      class="w-full mt-6"
      @submit="onSubmit"
    >
      <UFormField
        v-slot="{ error }"
        label="New Password"
        name="password"
        required
        :ui="{ error: 'text-red-500 text-sm mt-1' }"
      >
        <UInput
          v-model="password"
          type="password"
          :ui="{ base: 'py-4 px-6' }"
          :class="[
            'w-full transition-colors',
            error
              ? 'border-red-500 focus:border-red-500'
              : 'border-gray-300 focus:border-black',
          ]"
        />
      </UFormField>

      <UFormField
        v-slot="{ error }"
        label="Confirm Password"
        name="confirmPassword"
        required
        :ui="{ error: 'text-red-500 text-sm mt-1' }"
        class="mt-4"
      >
        <UInput
          v-model="confirmPassword"
          type="password"
          :ui="{ base: 'py-4 px-6' }"
          :class="[
            'w-full transition-colors',
            error
              ? 'border-red-500 focus:border-red-500'
              : 'border-gray-300 focus:border-black',
          ]"
        />
      </UFormField>

      <UButton
        type="submit"
        :loading="loading"
        class="flex justify-center items-center text-center w-full rounded py-4 text-white cursor-pointer mt-6"
      >
        Reset Password
      </UButton>
    </UForm>

    <p v-if="formError" class="text-red-500 text-sm mt-2">
      {{ formError }}
    </p>
  </div>
</template>

<style scoped></style>
