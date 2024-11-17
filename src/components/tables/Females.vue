<script setup lang="ts">
import { invoke, } from '@tauri-apps/api/core';
import { onMounted, type Ref, ref, inject } from 'vue';
import { RelativeIndividual } from '../../utils/types';

import { useStateStore } from '../../store/state';

const stateStore = useStateStore()

const relatives: Ref<Array<RelativeIndividual>> = ref([])
const hasActiveRelative = inject("hasActiveRelative") as Ref<boolean, boolean>

onMounted(() => {
  invoke("all_females").then(val => {
    console.log(val)
    const females = val as Array<RelativeIndividual>
    relatives.value = females

  })
})


function toggleNoteSection(id: number) {
  stateStore.changeActiveRelativeId(id)
  stateStore.setShowNotesToTrue()
  hasActiveRelative.value = true
}

</script>

<template>
  <h1>
    Females
  </h1>

  <div class="table-container">
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Age</th>
          <th>Sameness</th>
          <th>Swarthy</th>
          <th>Hotness</th>
          <th>Crazy</th>
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
          <td> {{ relative.swarthy || 0 }}</td>
          <td> {{ relative.hotness || 0 }}</td>
          <td> {{ relative.crazy || 0 }}</td>
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
</template>
