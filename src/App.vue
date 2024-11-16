<script setup lang="ts">
import { provide, type Ref, ref } from "vue";
import Aside from "./components/aside/index.vue"
import Females from "./components/tables/Females.vue";
import Relatives from "./components/tables/Relatives.vue";
import Employees from "./components/tables/Employees.vue";
import Modal from "./components/Modal.vue";
import EditForm from "./components/EditForm.vue";
import AddNewForm from "./components/AddNewForm.vue";
import NoteSection from "./components/NoteSection.vue";

const model_open = ref(false)
const active_tab: Ref<number> = ref(0)
provide("active_tab", active_tab)

const showNotes = ref(true)
provide("showNotes", showNotes)

const hasActiveRelative = ref(false)
provide("hasActiveRelative", hasActiveRelative)

const active_relative_id = ref<number>(0);
provide("active_relative_id", active_relative_id);

</script>

<template>
  <Modal :model_open="model_open" @close-modal="model_open = false" />
  <main class="container">
    <Aside />
    <NoteSection v-if="active_relative_id > 0 && showNotes && hasActiveRelative" />
    <div class="main">
      <div v-if="active_tab == 0">
        <Relatives />
      </div>
      <div v-if="active_tab == 1">
        <Females />
      </div>
      <div v-if="active_tab == 2">
        <Employees />
      </div>
      <div v-if="active_tab == 3">
        <AddNewForm />
      </div>
      <div v-if="active_tab == 4">
        <EditForm />
      </div>
    </div>
  </main>
</template>
