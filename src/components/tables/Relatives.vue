<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { Relative } from '../../utils/types';
import { onMounted, ref } from 'vue';

const relatives = ref<Relative[]>([])
const fetching = ref(true)
onMounted(() => {
  invoke("test_serde").then(val => {
    let relative: Relative = val as Relative
    relatives.value.push(relative)
    fetching.value = false
  })
})

</script>

<template>
  <h1>
    Relatives
  </h1>

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
        <tr v-for="relative in relatives">
          <td>{{ relative.name }}</td>
          <td>{{ relative.age || "" }}</td>
          <td> {{ relative.sameness || 0 }}</td>
          <td>{{ relative.mother || "" }}</td>
          <td>{{ relative.father || "" }}</td>
          <td>{{ relative.phone || "" }}</td>
          <td>{{ relative.email || "" }}</td>
          <td>{{ relative.pinned ? "pinned" : "not pinned" }}</td>
          <td>{{ relative.lostReason || "" }}</td>
          <td>{{ relative.created || "" }}</td>
          <td>{{ relative.updated || "" }}</td>
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
</style>
