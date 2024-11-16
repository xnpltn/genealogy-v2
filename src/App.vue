<script setup lang="ts">
import { provide, type Ref, ref } from "vue";
import Aside from "./components/aside/index.vue"
import Females from "./components/tables/Females.vue";
import Relatives from "./components/tables/Relatives.vue";
import Employees from "./components/tables/Employees.vue";
import Modal from "./components/Modal.vue";
import About from "./components/About.vue";
import AddNewForm from "./components/AddNewForm.vue";
import NoteSection from "./components/NoteSection.vue";

const active_tab: Ref<number> = ref(0)
provide("active_tab", active_tab)

const model_open = ref(false)
const showNotes = ref(true)
provide("showNotes", showNotes)


const active_relative_id = ref<number>(0);
provide("active_relative_id", active_relative_id);
//function open_modal() {
//  model_open.value = !model_open.value
//}



</script>

<template>

  <Modal :model_open="model_open" @close-modal="model_open = false" />
  <main class="container">
    <Aside />
    <NoteSection v-if="active_relative_id > 0 && showNotes" />
    <div class="main">

      <!--
      <h1>Hello wolrd</h1>
      <h1>{{ newRelative.name }}</h1>
      <button @click="test"> Click Me</button>
      <button @click="open_modal"> Open Modal</button>
      -->

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
        <About />
      </div>
    </div>

  </main>
</template>

<style scoped>
.main {
  margin-top: calc(var(--size-6xl) + var(--size-sm));
}
</style>
