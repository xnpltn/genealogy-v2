<script setup lang="ts">
import { onMounted, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { CreateRelativeParams } from '../utils/types';
import { useRelativesStore } from '../store/relatives';
import { useStateStore } from '../store/state';

const relativesStore = useRelativesStore()
const stateStore = useStateStore()


onMounted(() => {
  relativesStore.fetchMaleParents(stateStore.activeRelativeId)
  relativesStore.fetchFemaleParants(stateStore.activeRelativeId)
})

const newRelative = reactive({ sameness: 0, hotness: 0, crazy: 0, employable: 0, swarthy: 0, pinned: false, sex: 'male' }) as CreateRelativeParams;
function save() {
  let re = newRelative as CreateRelativeParams
  invoke("create_relative", { newRelative: re }).then(() => {
  }).catch(e => {
    console.log(e)
  })
}
</script>
<template>
  <div>
    <form @submit.prevent="save" id="createRelativeForm">
      <!-- Required Fields -->
      <div>
        <label for="firstName">First Name:</label>
        <input type="text" v-model="newRelative.firstName" id="firstName" name="firstName" required>
      </div>

      <div>
        <label for="lastName">Last Name:</label>
        <input type="text" v-model="newRelative.lastName" id="lastName" name="lastName" required>
      </div>

      <div>
        <label for="pinned">Pinned:</label>
        <input type="checkbox" v-model="newRelative.pinned" id="pinned" name="pinned">
      </div>

      <!-- Optional Fields -->
      <div>
        <label for="middleName">Middle Name:</label>
        <input type="text" v-model="newRelative.middleName" id="middleName" name="middleName">
      </div>

      <div>
        <label for="sex">Sex:</label>
        <select name="sex" id="sex" v-model="newRelative.sex">
          <option value="male">Male</option>
          <option value="female">Female</option>
        </select>
      </div>

      <div>
        <label for="birthday">Birthday:</label>
        <input type="date" v-model="newRelative.birthday" id="birthday" name="birthday">
      </div>
      <div>
        <label for="birthday">Died At:</label>
        <input type="date" v-model="newRelative.diedAt" id="birthday" name="birthday">
      </div>
      <div>
        <label for="sameness">Sameness:</label>
        <input type="number" id="sameness" v-model="newRelative.sameness" name="sameness" step="1" min="0" max="10"
          value="0.0">
      </div>

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

      <div>
        <label for="phone">Phone:</label>
        <input type="tel" id="phone" v-model="newRelative.phone" name="phone">
      </div>

      <div>
        <label for="email">Email:</label>
        <input type="email" id="email" v-model="newRelative.email" name="email">
      </div>
      <div>
        <label for="state">State:</label>
        <input type="text" id="state" v-model="newRelative.state" name="email">
      </div>
      <div>
        <label for="address">Addres:</label>
        <input type="text" id="address" v-model="newRelative.address" name="email">
      </div>

      <div>
        <label for="lostReason">Lost Reason:</label>
        <textarea id="lostReason" name="lostReason" v-model="newRelative.lostReason"></textarea>
      </div>

      <div v-if="newRelative.sex == 'female'">

        <div>
          <label for="swarthy">Swarthy:</label>
          <input type="number" id="swarthy" name="swarthy" v-model="newRelative.swarthy" step="1" min="0" max="10"
            value="0">
        </div>

        <div>
          <label for="hotness">Hotness:</label>
          <input type="number" id="hotness" name="hotness" v-model="newRelative.hotness" step="1" min="0" max="10"
            value="0">
        </div>

        <div>
          <label for="crazy">Crazy:</label>
          <input type="number" id="crazy" name="crazy" v-model="newRelative.crazy" step="1" min="0" max="10" value="0">
        </div>
      </div>

      <div v-if="newRelative.sex == 'male'">
        <label for="employable">Employable:</label>
        <input type="number" id="employable" v-model="newRelative.employable" name="employable" step="1" min="0"
          max="10" value="0">
      </div>
      <button type="submit">Submit</button>
    </form>

  </div>
</template>


<style>
form {
  background-color: #fff;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  width: 100%;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  color: #555;
  font-weight: bold;
}

input[type="text"],
input[type="date"],
input[type="number"],
input[type="tel"],
input[type="email"],
textarea {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  box-sizing: border-box;
  font-size: 14px;
}

/* Full-width fields */
textarea,
button {
  grid-column: span 2;
}

input[type="checkbox"] {
  margin-right: 5px;
}

textarea {
  resize: vertical;
  height: 80px;
}

button {
  padding: 10px;
  background-color: #007bff;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.3s;
}

button:hover {
  background-color: #0056b3;
}

/* Checkbox and Label Alignment */
label[for="pinned"] {
  display: flex;
  align-items: center;
  gap: 8px;
}

input:focus {
  border-color: #007bff;
  outline: none;
}
</style>
