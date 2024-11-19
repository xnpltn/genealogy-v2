<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { type RelativeIndividual } from '../../utils/types';
import { onMounted, type Ref, ref } from 'vue';
import { useStateStore } from '../../store/state';
import { useNotesStore } from '../../store/notes';
import { useFilesStore } from '../../store/files';

const notesStore = useNotesStore()
const filesStore = useFilesStore()
const stateStore = useStateStore()
const relatives: Ref<Array<RelativeIndividual>> = ref([])
const fetching = ref(true)


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

function resetState() {
  notesStore.activeNoteId = 0
  filesStore.activeFileId = 0
}
function toggleNoteSection(id: number) {
  stateStore.changeActiveRelativeId(id)
  stateStore.setShowNotesToTrue()
  resetState()
}
</script>

<template>
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
        <tr v-for="relative in relatives" :key="relative.id" @click="toggleNoteSection(relative.id)"
          :class="{ 'selected': stateStore.activeRelativeId == relative.id }">
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
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  border-radius: 8px;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-family: Arial, sans-serif;
  font-size: 13px;
}

h1 {
  text-align: center;
  color: #333;
  margin-bottom: 20px;
}

thead {
  background-color: #f8f9fa;
  position: sticky;
  top: 0;
  z-index: 10;
}

th {
  padding: 10px;
  text-align: left;
  border-bottom: 2px solid #e0e0e0;
  color: #555;
  font-weight: 600;
  text-transform: uppercase;
}

/*tbody tr {
  transition: background-color 0.3s ease;
  cursor: pointer;
}*/



td {
  padding: 8px 10px;
  border-bottom: 1px solid #e0e0e0;
  max-width: 150px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

td:nth-child(8) {
  font-weight: bold;
}

.selected {
  background-color: red;
}
</style>
