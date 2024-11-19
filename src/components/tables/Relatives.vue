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
          <td>{{ relative.age }}</td>
          <td> {{ relative.sameness }}</td>
          <td>{{ relative.mother }}</td>
          <td>{{ relative.father }}</td>
          <td>{{ relative.phone }}</td>
          <td>{{ relative.email }}</td>
          <td>{{ relative.pinned ? "pinned" : "not pinned" }}</td>
          <td>{{ relative.lostReason }}</td>
          <td>{{ relative.createdAt }}</td>
          <td>{{ relative.updatedAt }}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div v-else>
    Loading...
  </div>
</template>
