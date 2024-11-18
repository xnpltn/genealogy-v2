<script setup lang="ts">

import { onMounted, type Ref, ref, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { RelativeIndividual, CreateRelativeParams } from '../utils/types';
import { useStateStore } from '../store/state';
const activeRelative = ref({}) as Ref<RelativeIndividual, RelativeIndividual>
import { useRelativesStore } from '../store/relatives';

const relativesStore = useRelativesStore()
const stateStore = useStateStore()


const newRelative = reactive({}) as CreateRelativeParams;
newRelative.sameness = 0
newRelative.hotness = 0
newRelative.crazy = 0
newRelative.employable = 0
newRelative.swarthy = 0
newRelative.pinned = false
newRelative.sex = "male"

onMounted(() => {
  relativesStore.fetchMaleParents(stateStore.activeRelativeId)
  relativesStore.fetchFemaleParants(stateStore.activeRelativeId)
})

onMounted(() => {
  console.log(stateStore.activeRelativeId)
  invoke("relative_by_id", { id: stateStore.activeRelativeId }).then((val) => {
    activeRelative.value = val as RelativeIndividual
  }).catch(e => {
    console.log(e)
  })
})


function saveRelative() {
  invoke("update_relative", { relative: { ...activeRelative.value, ...newRelative } }).catch(e => {
    console.log(e)
  })
}

</script>

<template>
  <div class="preview">
    <div>
      <!-- left  part -->
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

        <div>
          <label for="mother">Mother:</label>
          <select name="mother" id="mother" v-model="newRelative.motherId">
            <option v-for="mother in relativesStore.femaleParents" :value="mother.id" :key=mother.id>
              {{ mother.firstName + ' ' + mother.lastName }}
            </option>
          </select>
        </div>
        <div>
          <label for="father">Father:</label>
          <select name="father" id="father" v-model="newRelative.fatherId">
            <option v-for="father in relativesStore.maleParents" :value="father.id" :key="father.id">
              {{ father.firstName + ' ' + father.lastName }}
            </option>
          </select>
        </div>

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
    <div>
      <!-- right part -->
      Hello World
    </div>
  </div>
</template>


<style scoped>
.preview {
  display: grid;
  grid-template-columns: 2fr 1fr;
  height: calc(100vh - 20%);
  gap: 20px;
  padding: 20px;
  background-color: #f4f7f6;
}

#createRelativeForm {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 15px;
  background-color: white;
  padding: 30px;
  border-radius: 10px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  overflow-y: auto;
  max-height: 100%;
}

#createRelativeForm>div,
#createRelativeForm>label,
#createRelativeForm>input,
#createRelativeForm>select,
#createRelativeForm>textarea {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

label {
  font-size: 0.9em;
  color: #495057;
  font-weight: 600;
}

input,
select,
textarea {
  padding: 8px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 0.9em;
}

input[type="checkbox"] {
  width: 20px;
  height: 20px;
}

button[type="submit"] {
  grid-column: 1 / -1;
  padding: 10px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button[type="submit"]:hover {
  background-color: #0056b3;
}

/* Right side text container */
.preview>div:last-child {
  background-color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  padding: 20px;
  text-align: center;
}
</style>
