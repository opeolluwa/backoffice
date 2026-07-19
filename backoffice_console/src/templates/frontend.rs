use super::backend::ModuleConfig;

pub fn store(cfg: &ModuleConfig) -> String {
    format!(
        r#"import api from "~/plugin/api";
import type {{ {name_pascal}Interface }} from "@bindings/{name_pascal}Interface";

const use{Name}Store = defineStore("{name}", {{
  state: () => ({{
    {plural}: [] as Array<{name_pascal}Interface>,
  }}),

  actions: {{
    async fetch{plural_pascal}() {{
      try {{
        const res = await api.get("/{plural}");
        this.{plural} = res.data?.data || [];
      }} catch (error) {{
        console.error("Failed to fetch {plural}:", error);
      }}
    }},
  }},
  persist: true,
}});

export {{ use{Name}Store }};
"#,
        name = cfg.name,
        name_pascal = cfg.name_pascal,
        plural = cfg.name_plural,
        plural_pascal = cfg.name_plural_pascal,
        Name = cfg.name_pascal,
    )
}

pub fn page(cfg: &ModuleConfig) -> String {
    format!(
        r#"<script setup lang="ts">
import type {{ FormSubmitEvent, TableColumn }} from "@nuxt/ui";
import * as v from "valibot";
import api from "~/plugin/api";
import {{ use{Name}Store }} from "~/stores/{name}";
import {{ h, resolveComponent }} from "vue";
import type {{ Row }} from "@tanstack/vue-table";
import {{ getPaginationRowModel }} from "@tanstack/vue-table";
import type {{ {name_pascal}Interface }} from "@bindings/{name_pascal}Interface";

useHead({{ title: "{name_pascal}" }});

definePageMeta({{
  layout: "dashboard",
  breadcrumb: {{
    icon: "{icon}",
    ariaLabel: "{name_pascal}",
    title: "{name_pascal}",
  }},
}});

const UButton = resolveComponent("UButton");
const UDropdownMenu = resolveComponent("UDropdownMenu");

const toast = useToast();

const columns: TableColumn<{name_pascal}Interface>[] = [
  {{
    accessorKey: "name",
    header: "Name",
    cell: ({{ row }}) => `${{row.getValue("name")}}`,
  }},
  {{
    accessorKey: "description",
    header: "Description",
    cell: ({{ row }}) => `${{row.getValue("description")}}`,
  }},
  {{
    accessorKey: "createdAt",
    header: "Date created",
    cell: ({{ row }}) => {{
      return new Date(row.getValue("createdAt")).toLocaleString("en-US", {{
        day: "numeric",
        month: "short",
        hour: "2-digit",
        minute: "2-digit",
        hour12: false,
      }});
    }},
  }},
  {{
    id: "actions",
    cell: ({{ row }}) => {{
      return h(
        "div",
        {{ class: "text-right text-red-500" }},
        h(
          UDropdownMenu,
          {{
            content: {{
              align: "end",
            }},
            items: getRowItems(row),
            "aria-label": "Actions dropdown",
          }},
          () =>
            h(UButton, {{
              icon: "i-lucide-ellipsis-vertical",
              color: "neutral",
              variant: "ghost",
              class: "ml-auto",
              "aria-label": "Actions dropdown",
            }}),
        ),
      );
    }},
  }},
];

function getRowItems(row: Row<{name_pascal}Interface>) {{
  const router = useRouter();
  const {name}Store = use{Name}Store();
  const identifier = row.original.identifier;

  return [
    {{
      type: "label",
      label: "Actions",
    }},
    {{ type: "separator" }},
    {{
      label: "Update",
      icon: "i-lucide-pencil",
      onSelect() {{
        router.push(`/{plural}/${{identifier}}/update`);
      }},
    }},
    {{
      label: "Delete",
      icon: "i-lucide-trash",
      class: "text-red-500",
      async onSelect() {{
        try {{
          await api.delete(`/{plural}/${{identifier}}`);
          toast.add({{
            title: "Deleted",
            description: "{name_pascal} deleted successfully.",
            color: "success",
          }});
          await {name}Store.fetch{plural_pascal}();
        }} catch {{
          toast.add({{
            title: "Error",
            description: "Failed to delete {name}.",
            color: "error",
          }});
        }}
      }},
    }},
  ];
}}

const schema = v.object({{
  name: v.pipe(v.string(), v.minLength(1, "Name is required")),
  description: v.pipe(v.string(), v.minLength(1, "Description is required")),
}});

type Schema = v.InferOutput<typeof schema>;

const openForm = ref(false);
const state = reactive<Schema>({{
  name: "",
  description: "",
}});

const resetForm = () => {{
  state.name = "";
  state.description = "";
}};

const {name}Store = use{Name}Store();
const fetchingItems = ref(false);

const items = ref<{name_pascal}Interface[]>();
const nullItems = computed(() => !items.value?.length);

const loading = ref(false);
async function onSubmit({{ data }}: FormSubmitEvent<Schema>) {{
  loading.value = true;
  try {{
    const res = await api.post("/{plural}", data);
    if (res.status !== 201) {{
      throw new Error(res.data?.message || "Failed to create {name}");
    }}
    toast.add({{
      title: "Success",
      description: "{name_pascal} created successfully.",
    }});
    openForm.value = false;
    resetForm();
  }} catch {{
    toast.add({{
      title: "Error",
      description: "Failed to create {name}. Please try again.",
      color: "error",
    }});
  }} finally {{
    loading.value = false;
    await {name}Store.fetch{plural_pascal}();
    items.value = {name}Store.{plural};
  }}
}}

onMounted(async () => {{
  try {{
    await {name}Store.fetch{plural_pascal}();
    items.value = {name}Store.{plural};
  }} catch {{
    toast.add({{
      title: "Error",
      description: "Failed to load {plural}. Please try again.",
      color: "error",
    }});
  }} finally {{
    fetchingItems.value = false;
  }}
}});

const pagination = ref({{ pageIndex: 0, pageSize: 10 }});
const search = ref("");

const filteredItems = computed(() => {{
  const query = search.value.trim().toLowerCase();
  return (items.value || []).filter((item) => {{
    const name = item.name?.toLowerCase() || "";
    const description = item.description?.toLowerCase() || "";
    return !query || name.includes(query) || description.includes(query);
  }});
}});

const table = useTemplateRef("table");
</script>

<template>
  <div>
    <PageLoader v-if="fetchingItems" />

    <AppEmptyState
      v-if="nullItems"
      icon="{icon}"
      title="No {plural} yet"
      description="Create your first {name} to get started."
      action-label="Create first {name}"
      @action="openForm = true"
    />

    <div v-else>
      <div class="flex flex-col lg:flex-row gap-3 mb-5 px-4 py-3 border rounded border-accented items-end">
        <UInput
          v-model="search"
          class="max-w-sm"
          placeholder="Search by name / description"
        />
      </div>

      <UTable
        ref="table"
        v-model:pagination="pagination"
        :data="filteredItems"
        :loading="fetchingItems"
        loading-animation="carousel"
        :columns="columns"
        sticky="header"
        :pagination-options="{{
          getPaginationRowModel: getPaginationRowModel(),
        }}"
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
      title="Create {name_pascal}"
      description="A {name} lets you manage your data"
      close-icon="heroicons:x-mark"
    >
      <template #body>
        <UForm
          class="space-y-4"
          :schema="schema"
          :state="state"
          :on-submit="onSubmit"
        >
          <UFormField
            v-slot="{{{{ error }}}}"
            label="Name"
            name="name"
            required
            :ui="{{{{ error: 'text-red-500 text-sm mt-1' }}}}"
          >
            <UInput
              v-model="state.name"
              :ui="{{{{ base: 'py-4 px-6' }}}}"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <UFormField
            v-slot="{{{{ error }}}}"
            label="Description"
            name="description"
            required
            :ui="{{{{ error: 'text-red-500 text-sm mt-1' }}}}"
          >
            <UInput
              v-model="state.description"
              :ui="{{{{ base: 'py-4 px-6' }}}}"
              :class="[
                'w-full transition-colors',
                error
                  ? 'border-red-500 focus:border-red-500'
                  : 'border-gray-300 focus:border-black',
              ]"
            />
          </UFormField>

          <div class="flex justify-between items-center">
            <UButton
              type="submit"
              class="dark:text-white/90 py-3 px-4"
              :loading="loading"
              :disabled="loading"
            >
              Continue
            </UButton>
            <UButton
              variant="subtle"
              color="muted"
              class="dark:text-white/90 py-3 px-4"
              @click="resetForm"
            >
              Clear form
            </UButton>
          </div>
        </UForm>
      </template>
    </UModal>

    <AppContentButton
      v-show="nullItems === false"
      class="fixed bottom-12 right-20"
      @click="openForm = true"
    />
  </div>
</template>
"#,
        name = cfg.name,
        name_pascal = cfg.name_pascal,
        plural = cfg.name_plural,
        plural_pascal = cfg.name_plural_pascal,
        Name = cfg.name_pascal,
        icon = "heroicons:puzzle-piece",
    )
}
