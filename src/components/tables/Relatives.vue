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
  <div class="relatives-table"
    :class="{ 'relatives-table--light': !stateStore.darkTheme, 'relatives-table--dark': stateStore.darkTheme }">
    <table class="data-grid">
      <thead class="data-grid__header"
        :class="{ 'data-grid__header--light': !stateStore.darkTheme, 'data-grid__header--dark': stateStore.darkTheme }">
        <tr>
          <th class="data-grid__header-cell">Name</th>
          <th class="data-grid__header-cell">Age</th>
          <th class="data-grid__header-cell">Sameness</th>
          <th class="data-grid__header-cell">Mother</th>
          <th class="data-grid__header-cell">Father</th>
          <th class="data-grid__header-cell">Phone</th>
          <th class="data-grid__header-cell">Email</th>
          <th class="data-grid__header-cell">Pinned</th>
          <th class="data-grid__header-cell">Lost Reason</th>
          <th class="data-grid__header-cell">Created</th>
          <th class="data-grid__header-cell">Updated</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="relative in relatives" :key="relative.id" @click="toggleNoteSection(relative.id)"
          class="data-grid__row" :class="{
            'data-grid__row--light': !stateStore.darkTheme,
            'data-grid__row--dark': stateStore.darkTheme,
            'data-grid__row--selected': stateStore.activeRelativeId == relative.id
          }">
          <td class="data-grid__cell">{{ relative.firstName + ' ' + relative.lastName }}</td>
          <td class="data-grid__cell">{{ relative.age }}</td>
          <td class="data-grid__cell">{{ relative.sameness }}</td>
          <td class="data-grid__cell">{{ relative.mother }}</td>
          <td class="data-grid__cell">{{ relative.father }}</td>
          <td class="data-grid__cell">{{ relative.phone }}</td>
          <td class="data-grid__cell">{{ relative.email }}</td>
          <td class="data-grid__cell data-grid__cell--pinned">{{ relative.pinned ? "pinned" : "not pinned" }}</td>
          <td class="data-grid__cell">{{ relative.lostReason }}</td>
          <td class="data-grid__cell">{{ relative.createdAt }}</td>
          <td class="data-grid__cell">{{ relative.updatedAt }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped></style>
