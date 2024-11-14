<script setup lang="ts">
import { provide, type Ref, ref } from "vue";
import Aside from "./components/aside/index.vue"
import Females from "./components/tables/Females.vue";
import Relatives from "./components/tables/Relatives.vue";
import Employees from "./components/tables/Employees.vue";
import Modal from "./components/Modal.vue";
import About from "./components/About.vue";
import AddNewForm from "./components/AddNewForm.vue";

let text = ref("")

const active_tab: Ref<number> = ref(0)
provide("active_tab", active_tab)

const model_open = ref(false)

//function open_modal() {
//  model_open.value = !model_open.value
//}



</script>

<template>

  <Modal :model_open="model_open" @close-modal="model_open = false" />
  <main class="container">
    <Aside />
    <div class="main">

      <!--
      <h1>Hello wolrd</h1>
      <h1>{{ newRelative.name }}</h1>
      <button @click="test"> Click Me</button>
      <button @click="open_modal"> Open Modal</button>
      -->
      <h1>{{ text }}</h1>
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

  margin-left: 300px;
}
</style>
