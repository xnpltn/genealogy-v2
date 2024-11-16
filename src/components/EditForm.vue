<script setup lang="ts">

import { inject, onMounted, type Ref, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { RelativeIndividual } from '../utils/types';
const activeRelativeId = inject("active_relative_id") as Ref<number, number>
const activeRelative = ref({}) as Ref<RelativeIndividual, RelativeIndividual>

onMounted(() => {
  invoke("relative_by_id", { id: activeRelativeId.value }).then((val) => {
    activeRelative.value = val as RelativeIndividual
  }).catch(e => {
    console.log(e)
  })
})


function saveRelative() {
  invoke("update_relative", { relative: activeRelative.value }).catch(e => {
    console.log(e)
  })
}

</script>

<template>
  <div class="settings">
    <div>
      <form @submit.prevent="saveRelative" id="createRelativeForm">
        <!-- Required Fields -->
        <label for="firstName">First Name:</label>
        <input type="text" v-model="activeRelative.firstName" id="firstName" name="firstName" required>

        <label for="lastName">Last Name:</label>
        <input type="text" v-model="activeRelative.lastName" id="lastName" name="lastName" required>

        <label for="pinned">Pinned:</label>
        <input type="checkbox" v-model="activeRelative.pinned" id="pinned" name="pinned">

        <!-- Optional Fields -->
        <label for="middleName">Middle Name:</label>
        <input type="text" v-model="activeRelative.middleName" id="middleName" name="middleName">

        <label for="sex">Sex:</label>
        <select name="sex" id="sex" v-model="activeRelative.sex">
          <option value="male">Male</option>
          <option value="female">Female</option>
        </select>

        <label for="birthday">Birthday:</label>

        <input type="date" v-model="activeRelative.birthday" id="birthday" name="birthday">

        <label for="sameness">Sameness:</label>
        <input type="number" id="sameness" v-model="activeRelative.sameness" name="sameness" step="1" min="0" max="10"
          value="0.0">

        <label for="mother">Mother:</label>
        <input type="text" id="mother" v-model="activeRelative.mother" name="mother">

        <label for="father">Father:</label>
        <input type="text" id="father" v-model="activeRelative.father" name="father">

        <label for="phone">Phone:</label>
        <input type="tel" id="phone" v-model="activeRelative.phone" name="phone">

        <label for="email">Email:</label>
        <input type="email" id="email" v-model="activeRelative.email" name="email">

        <label for="lostReason">Lost Reason:</label>
        <textarea id="lostReason" name="lostReason" v-model="activeRelative.lostReason"></textarea>

        <div v-if="activeRelative.sex == 'female'">

          <label for="swarthy">Swarthy:</label>
          <input type="number" id="swarthy" name="swarthy" v-model="activeRelative.swarthy" step="1" min="0" max="10"
            value="0">

          <label for="hotness">Hotness:</label>
          <input type="number" id="hotness" name="hotness" v-model="activeRelative.hotness" step="1" min="0" max="10"
            value="0">

          <label for="crazy">Crazy:</label>
          <input type="number" id="crazy" name="crazy" v-model="activeRelative.crazy" step="1" min="0" max="10"
            value="0">
        </div>

        <div v-if="activeRelative.sex == 'male'">
          <label for="employable">Employable:</label>
          <input type="number" id="employable" v-model="activeRelative.employable" name="employable" step="1" min="0"
            max="10" value="0">
        </div>
        <button type="submit">Submit</button>
      </form>
    </div>
  </div>
</template>


<style scoped>
.settings {
  height: calc(100vh - 20%);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
}
</style>
