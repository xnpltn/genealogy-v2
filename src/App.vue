<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
//import { listen } from '@tauri-apps/api/event';
import { provide, reactive, type Ref, ref } from "vue";
import Aside from "./components/aside/index.vue"
import Females from "./components/tables/Females.vue";
import Relatives from "./components/tables/Relatives.vue";
import Employees from "./components/tables/Employees.vue";
import { Relative } from "./utils/types";
import Modal from "./components/Modal.vue";
import About from "./components/About.vue";

let text = ref("")
//async function test() {
//  text.value = await invoke("my_custom_command")
//  invoke("test_serde").then(val => {
//    let relative: Relative = val as Relative
//    console.log(relative)
//  })
//  invoke("download", { url: "URL" }).then(async () => {
//    await listen("test", (event) => {
//      console.log(event.payload)
//    })
//
//  })
//}


const active_tab: Ref<number> = ref(0)
provide("active_tab", active_tab)

const model_open = ref(false)

//function open_modal() {
//  model_open.value = !model_open.value
//}

const newRelative = reactive<Record<string, any>>({});

function save(){
  let re = newRelative as Relative
  re.id = 1
  re.pinned = false
  re.email = "me@mail.com"
invoke("save_new_relative", {newRelative: re}).then(()=>{
  alert("saved");
}).catch(e=>{
console.log(e)
})
}

</script>

<template>
  
  <Modal :model_open="model_open" @close-modal="model_open = false"/>
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
        <form @submit.prevent="save">
          <input type="text" v-model="newRelative.name" placeholder="name"/>
          <button type="submit">Submit</button>
        </form>
      </div>
      <div v-if="active_tab == 4"> 
        <About/>
      </div>
    </div>

  </main>
</template>

<style scoped>
.main {

  margin-left: 300px;
}


</style>
