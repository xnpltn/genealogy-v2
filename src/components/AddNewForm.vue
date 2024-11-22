<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import type { CreateRelativeParams } from '../utils/types';
import { useRelativesStore } from '../store/relatives';
import { useStateStore } from '../store/state';
import Modal from './Modal.vue';
import { open } from '@tauri-apps/plugin-dialog';

const relativesStore = useRelativesStore()
const stateStore = useStateStore()
const chosenImage = ref("")

onMounted(() => {
  relativesStore.fetchMaleParents(stateStore.activeRelativeId)
  relativesStore.fetchFemaleParants(stateStore.activeRelativeId)
})



const newRelative = reactive({ sameness: 0, hotness: 0, crazy: 0, employable: 0, swarthy: 0, pinned: false, sex: 'male' }) as CreateRelativeParams;
function save() {
  let re = newRelative as CreateRelativeParams
  invoke("create_relative", { newRelative: re }).then(() => {
    stateStore.activeTab = 0
  }).catch(e => {
    stateStore.errorValue = `${e}`
    stateStore.showError = true
    console.log(e)
  })
}

function uploadImage() {

  open({ multiple: false, directory: false, filters: [{ name: 'Image', extensions: ['jpg', 'png'] }] }).then((file) => {
    if (file) {
      chosenImage.value = file
    }
  })
}
</script>
<template>
  <Modal class="error_modal" :model_open="stateStore.showError"
    @close-modal="stateStore.showError = false; stateStore.errorValue = ''">
    {{ stateStore.errorValue }}
  </Modal>
  <div class="container">
    <div>
      <form @submit.prevent="save" id="createRelativeForm">
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
          <input type="text" id="email" v-model="newRelative.email" name="email">
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
          <select name="lostReason" id="lostReason" v-model="newRelative.lostReason">
            <option value="reason 1">Reason 1</option>
            <option value="reason 2">Reason 2</option>
            <option value="reason 3">Reason 3</option>
            <option value="reason 4">Reason 4</option>
            <option value="reason 5">Reason 5</option>
          </select>
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
            <input type="number" id="crazy" name="crazy" v-model="newRelative.crazy" step="1" min="0" max="10"
              value="0">
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
    <div class="image__container">
      <div style="height: 200px; width: 200px; background-color: red;">
        <img v-if="chosenImage.length" :src="convertFileSrc(chosenImage)" alt="">

      </div>
      <button @click="uploadImage">Upload Image</button>
    </div>
  </div>
</template>


<style scoped>
.container {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 20px;
  padding: 20px;
  background-color: #f8f9fa;
  height: calc(100vh - 20%);
}

#createRelativeForm {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 15px;
  background-color: #fff;
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  overflow-y: auto;
}

#createRelativeForm>div {
  display: flex;
  flex-direction: column;
}

label {
  font-size: 0.9em;
  color: #495057;
  font-weight: bold;
  margin-bottom: 5px;
}

input,
select,
textarea {
  padding: 8px 12px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 1em;
  outline: none;
  transition: border-color 0.3s;
}

input:focus,
select:focus,
textarea:focus {
  border-color: #007bff;
}

button[type="submit"] {
  grid-column: 1 / -1;
  padding: 10px;
  background-color: #007bff;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 1em;
  cursor: pointer;
  transition: background-color 0.3s, transform 0.3s;
}

button[type="submit"]:hover {
  background-color: #0056b3;
  transform: scale(1.02);
}

.delete_button {
  padding: 8px 16px;
  background-color: #dc3545;
  color: #fff;
  border: none;
  border-radius: 4px;
  font-size: 0.9em;
  cursor: pointer;
  transition: background-color 0.3s, transform 0.3s;
}

.delete_button:hover {
  background-color: #bd2130;
  transform: scale(1.05);
}

.image__container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15px;
  background-color: #fff;
  padding: 20px;
  border-radius: 10px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.image__container img {
  max-height: 200px;
  max-width: 200px;
  border: 2px solid #ddd;
  border-radius: 10px;
  object-fit: cover;
}

table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 10px;
}

th,
td {
  padding: 10px;
  text-align: left;
  border-bottom: 1px solid #ddd;
}

tr:hover {
  background-color: #f1f1f1;
}

th {
  background-color: #007bff;
  color: white;
}

@media (max-width: 768px) {
  .container {
    grid-template-columns: 1fr;
  }

  #createRelativeForm {
    padding: 15px;
  }

  .image__container {
    padding: 15px;
  }
}
</style>
