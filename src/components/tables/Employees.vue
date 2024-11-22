<script setup lang="ts">
import { invoke, } from '@tauri-apps/api/core';
import { onMounted, type Ref, ref } from 'vue';
import { useStateStore } from '../../store/state';
import { type RelativeIndividual } from '../../utils/types';

const relatives: Ref<Array<RelativeIndividual>> = ref([])
const stateStore = useStateStore()

onMounted(() => {
  invoke("all_employees").then(val => {
    console.log(val)
    const females = val as Array<RelativeIndividual>
    relatives.value = females
  })
})

function toggleNoteSection(id: number) {
  stateStore.changeActiveRelativeId(id)
  stateStore.setShowNotesToTrue()
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
          <th class="data-grid__header-cell">Employable</th>
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
          <td class="data-grid__cell">{{ relative.employable }}</td>
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
