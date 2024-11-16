<script setup lang="ts">
//import UpdateForm from '../UpdateForm.vue';
//import Modal from '../Modal.vue';
import { invoke } from '@tauri-apps/api/core';
import { RelativeIndividual } from '../../utils/types';
import { inject, onMounted, type Ref, ref } from 'vue';

const relatives: Ref<Array<RelativeIndividual>> = ref([])
const fetching = ref(true)
const active_relative_id = inject("active_relative_id") as Ref<number, number>;
const showNotes = inject<Ref<Boolean>>("showNotes") as Ref<boolean, boolean>;
const hasActiveRelative = inject("hasActiveRelative") as Ref<boolean, boolean>
onMounted(() => {
  invoke("all_relatives").then((val) => {
    relatives.value = val as Array<RelativeIndividual>;
    fetching.value = false
  }).catch((e) => {
    if (e instanceof Error) {
      console.log(e.message, e.name, e.stack)
    } else {
      console.log(e)
    }
  })
})

//function open_update_modal(id: number) {
//  active_relative_id.value = id
//  console.log(active_relative_id.value)
//}


function toggleNoteSection(id: number) {
  hasActiveRelative.value = true
  active_relative_id.value = id
  showNotes.value = true
}



</script>

<template>
  <h1>
    Relatives
  </h1>

  <!--<Modal @close-modal="model_open = false" :model_open="model_open">
    <UpdateForm :relative="active_relative" />
  </Modal>
-->
  <div class="table-container" v-if="!fetching">
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Age</th>
          <th>Sameness</th>
          <th>Mother</th>
          <th>Father</th>
          <th>Phone</th>
          <th>Email</th>
          <th>Pinned</th>
          <th>Lost Reason</th>
          <th>Created</th>
          <th>Updated</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="relative in relatives" :key="relative.id" @click="toggleNoteSection(relative.id)">
          <td>{{ relative.firstName + ' ' + relative.lastName }}</td>
          <td>{{ relative.age || "" }}</td>
          <td> {{ relative.sameness || 0 }}</td>
          <td>{{ relative.mother || "" }}</td>
          <td>{{ relative.father || "" }}</td>
          <td>{{ relative.phone || "" }}</td>
          <td>{{ relative.email || "" }}</td>
          <td>{{ relative.pinned ? "pinned" : "not pinned" }}</td>
          <td>{{ relative.lostReason || "" }}</td>
          <td>{{ relative.createdAt || "" }}</td>
          <td>{{ relative.updatedAt || "" }}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div v-else>
    Loading...
  </div>
</template>


<style scoped>
.table-container {
  width: 100%;
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
  text-align: left;
}

th,
td {
  padding: 10px;
  border: 1px solid #000;
}

th {
  background-color: #e6f2ff;
}

tr:nth-child(even) td {
  background-color: #f9f9f9;
}

.highlighted {
  background-color: #ffe5b4;
}

tr {
  cursor: pointer;
}
</style>
