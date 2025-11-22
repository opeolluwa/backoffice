<script setup lang="ts">
import type { FormSubmitEvent, TableColumn } from "@nuxt/ui";
import * as v from "valibot";
import api from "~/plugin/api";
import { useMarketplaceStore } from "~/stores/marketplace";
import type { MarketPlace } from "~/types/Marketplace";
import { h, resolveComponent } from "vue";
import type { Row } from "@tanstack/vue-table";
import { useClipboard } from "@vueuse/core";
import { getPaginationRowModel } from "@tanstack/vue-table";

definePageMeta({
  layout: "dashboard",
});

const UButton = resolveComponent("UButton");
const UDropdownMenu = resolveComponent("UDropdownMenu");

const toast = useToast();
const { copy } = useClipboard();

const columns: TableColumn<MarketPlace>[] = [
  // {
  //   accessorKey: "identifier",
  //   header: "#",
  //   cell: ({ row }) => `#${row.getValue("identifier")}`,
  // },

  {
    accessorKey: "name",
    header: "Name",
    cell: ({ row }) => `${row.getValue("name")}`,
  },

  {
    accessorKey: "slug",
    header: "Slug",
    cell: ({ row }) => `#${row.getValue("slug")}`,
  },

  {
    accessorKey: "createdAt",
    header: "Date created",
    cell: ({ row }) => {
      return new Date(row.getValue("createdAt")).toLocaleString("en-US", {
        day: "numeric",
        month: "short",
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      });
    },
  },
  {
    id: "actions",
    cell: ({ row }) => {
      return h(
        "div",
        { class: "text-right" },
        h(
          UDropdownMenu,
          {
            content: {
              align: "end",
            },
            items: getRowItems(row),
            "aria-label": "Actions dropdown",
          },
          () =>
            h(UButton, {
              icon: "i-lucide-ellipsis-vertical",
              color: "neutral",
              variant: "ghost",
              class: "ml-auto",
              "aria-label": "Actions dropdown",
            }),
        ),
      );
    },
  },
];

function getRowItems(row: Row<MarketPlace>) {
  const router = useRouter();
  const marketplaceStore = useMarketplaceStore();
  // const { confirm } = useModal(); // Nuxt UI modal composable

  const identifier = row.original.identifier;

  return [
    {
      type: "label",
      label: "Actions",
    },

    {
      label: "Copy payment ID",
      onSelect() {
        copy(identifier);

        toast.add({
          title: "Payment ID copied!",
          color: "success",
          icon: "i-lucide-circle-check",
        });
      },
    },

    { type: "separator" },

    // --- NEW CLEAN ACTIONS ---
    {
      label: "View entries",
      icon: "i-lucide-list",
      onSelect() {
        router.push(`/marketplace/${identifier}/products`);
      },
    },

    {
      label: "Update",
      icon: "i-lucide-pencil",
      onSelect() {
        router.push(`/marketplace/${identifier}/update`);
      },
    },

    {
      label: "Delete",
      icon: "i-lucide-trash",
      class: "text-red-500",
      async onSelect() {
        // const ok = await confirm({
        //   title: "Delete marketplace?",
        //   description: "This action cannot be undone.",
        //   confirmButton: {
        //     label: "Delete",
        //     color: "error",
        //   },
        //   cancelButton: {
        //     label: "Cancel",
        //   },
        // });

        // if (!ok) return;

        try {
          await api.delete(`/marketplaces/${identifier}`);

          toast.add({
            title: "Deleted",
            description: "Marketplace deleted successfully.",
            color: "success",
          });

          await marketplaceStore.fetchMarketplaces();
        } catch {
          toast.add({
            title: "Error",
            description: "Failed to delete marketplace.",
            color: "error",
          });
        }
      },
    },
  ];
}

const schema = v.object({
  name: v.pipe(v.string(), v.minLength(1, "Name is required ")),
  description: v.pipe(v.string(), v.minLength(1, "Description is required ")),
  slug: v.pipe(v.string(), v.minLength(1, "Slug is required ")),
});

type Schema = v.InferOutput<typeof schema>;

const openForm = ref(false);
const state = reactive<Schema>({
  name: "",
  description: "",
  slug: "",
});

const resetForm = () => {
  state.name = "";
  state.description = "";
  state.slug = "";
};

const marketplaceStore = useMarketplaceStore();
const fetchingMarketplaces = ref(false);

const marketplaces = ref<MarketPlace[]>();
const nullMarketplaces = computed(() => !marketplaces.value?.length);

const loading = ref(false);
async function onSubmit({ data }: FormSubmitEvent<Schema>) {
  loading.value = true;
  try {
    const res = await api.post("/marketplaces", data);
    if (res.status !== 201) {
      throw new Error(res.data?.message || "Failed to create marketplace");
    }
    toast.add({
      title: "Success",
      description: "Product added successfully.",
    });
    openForm.value = false;
    resetForm();
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to create marketplace. Please try again.",
      color: "error",
    });
  } finally {
    loading.value = false;
    await marketplaceStore.fetchMarketplaces();
    marketplaces.value = marketplaceStore.marketplaces;
  }
}

onMounted(async () => {
  try {
    await marketplaceStore.fetchMarketplaces();
    marketplaces.value = marketplaceStore.marketplaces;
  } catch {
    toast.add({
      title: "Error",
      description: "Failed to load marketplaces. Please try again.",
      color: "error",
    });
  } finally {
    fetchingMarketplaces.value = false;
  }
});

const globalFilter = ref("");
const pagination = ref({
  pageIndex: 0,
  pageSize: 5,
});
const table = useTemplateRef("table");
</script>

<template>
  <div>
    <PageLoader v-if="fetchingMarketplaces" />

    <div
      v-if="nullMarketplaces"
      class="flex flex-col justify-center items-center h-[70vh]"
    >
      <h1>You currently don&apos;t have any product</h1>

      <UButton
        color="neutral"
        variant="outline"
        class="px-5 py-4 rounded mt-3 cursor-pointer flex items-center justify-center gap-x-2"
        @click="openForm = true"
      >
        <UIcon name="heroicons:plus-circle" class="size-5" />
        Add product
      </UButton>
    </div>

    <div v-else>
      <h1 class="justify-start">#Marketplaces</h1>
      <div class="justify-between items-center hidden">
        <div class="flex px-4 py-3.5 border-accented">
          <UInput
            ref="table"
            v-model="globalFilter"
            v-model:global-filter="globalFilter"
            class="max-w-sm"
            placeholder="Filter..."
          />
        </div>
      </div>
      <UTable
        ref="table"
        v-model:pagination="pagination"
        :data="marketplaces"
        class=""
        :loading="fetchingMarketplaces"
        loading-animation="carousel"
        :columns="columns"
        sticky="header"
        :pagination-options="{
          getPaginationRowModel: getPaginationRowModel(),
        }"
      />

      <div class="flex justify-center border-t border-default pt-4 mt-6">
        <UPagination
          :default-page="
            (table?.tableApi?.getState().pagination.pageIndex || 0) + 1
          "
          :items-per-page="table?.tableApi?.getState().pagination.pageSize"
          :total="table?.tableApi?.getFilteredRowModel().rows.length"
          @update:page="(p) => table?.tableApi?.setPageIndex(p - 1)"
        />
      </div>
    </div>

    <UModal
      v-model:open="openForm"
      title="Create store"
      description="A store lets you manage your goods"
      close-icon="heroicons:x-mark"
    >
      <template #body>
        <UForm
          class="space-y-4"
          :schema="schema"
          :state="state"
          @submit.prevent="onSubmit"
        >
          <UFormField
            v-slot="{ error }"
            label="Name"
            name="name"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          >
            <UInput
              v-model="state.name"
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
            label="Description"
            name="description"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          >
            <UInput
              v-model="state.description"
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
            label="Slug"
            name="slug"
            hint="eg: cars, roses"
            required
            :ui="{ error: 'text-red-500 text-sm mt-1' }"
          >
            <UInput
              v-model="state.slug"
              :ui="{ base: 'py-4 px-6' }"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <div class="flex justify-between items-center">
            <UButton type="submit" :loading="loading" :disabled="loading">
              Continue
            </UButton>
            <UButton variant="subtle" color="error" @click="resetForm">
              Clear form
            </UButton>
          </div>
        </UForm>
      </template>
    </UModal>

    <UButton
      icon="heroicons:plus-20-solid"
      size="md"
      color="primary"
      variant="solid"
      class="fixed bottom-12 right-20"
      @click="openForm = true"
    />
  </div>
</template>

<style scoped></style>
