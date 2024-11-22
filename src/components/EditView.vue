<script setup lang="ts">

import { onMounted, type Ref, ref, reactive, watchEffect } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { type RelativeIndividual, type CreateRelativeParams } from '../utils/types';
import { confirm, open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';
import { useStateStore } from '../store/state';
import { useRelativesStore } from '../store/relatives';
import { useImagesStore } from '../store/images';

const relativesStore = useRelativesStore()
const stateStore = useStateStore()
const imageStore = useImagesStore()
const activeRelative = ref({}) as Ref<RelativeIndividual, RelativeIndividual>
const newRelative = reactive({}) as CreateRelativeParams;
const removeMother = ref(false)
const removeFather = ref(false)
const chosenImage = ref('')

onMounted(() => {
  relativesStore.fetchMaleParents(stateStore.activeRelativeId)
  relativesStore.fetchFemaleParants(stateStore.activeRelativeId)
})

watchEffect(() => {
  newRelative.sex = activeRelative.value.sex
})

onMounted(() => {
  console.log(stateStore.activeRelativeId)
  invoke("relative_by_id", { id: stateStore.activeRelativeId }).then((val) => {
    activeRelative.value = val as RelativeIndividual
  }).catch(e => {
    console.log(e)
  })
  imageStore.loadImages(stateStore.activeRelativeId)
})


function saveRelative() {
  if (newRelative.sex.toLowerCase() == 'female') {
    newRelative.employable = 0
  } else if (newRelative.sex.toLowerCase() == 'male') {
    newRelative.hotness = 0
    newRelative.crazy = 0
    newRelative.swarthy = 0
  }

  if (removeFather.value) {
    newRelative.fatherId = null
  }
  if (removeMother.value) {
    newRelative.motherId = null
  }
  invoke("update_relative", { relative: { ...activeRelative.value, ...newRelative } }).then(() => {
    stateStore.activeTab = 0
  }).catch(e => {
    console.log(e)
  })
}

function deleteRelative() {
  confirm("This action is irreversible, are you sure?", { kind: 'warning' }).then(y => {
    if (y) {
      invoke("delete_relative", { relativeId: stateStore.activeRelativeId }).then(() => {
        stateStore.activeTab = 0
      }).catch(e => {
        console.log(e)
      })
    }
  }).catch(e => {
    console.log(e)
  })
}

function uploadImage() {
  open({ multiple: false, directory: false, filters: [{ name: 'Image', extensions: ['jpg', 'png', 'jpeg', 'webp'] }] }).then((file) => {
    if (file) {
      chosenImage.value = file
      imageStore.createImage(chosenImage.value, stateStore.activeRelativeId)
      imageStore.loadImages(stateStore.activeRelativeId)
    }
  })
}


</script>

<template>
  <div class="btn-container">
    <button @click="deleteRelative" class="delete_button">Delete</button>
  </div>
  <div class="container">
    <div>
      <form @submit.prevent="saveRelative" id="createRelativeForm">
        <div>
          <label for="firstName">First Name:</label>
          <input type="text" v-model="activeRelative.firstName" id="firstName" name="firstName" required>
        </div>

        <div>
          <label for="lastName">Last Name:</label>
          <input type="text" v-model="activeRelative.lastName" id="lastName" name="lastName" required>
        </div>

        <div>
          <label for="pinned">Pinned:</label>
          <input type="checkbox" v-model="activeRelative.pinned" id="pinned" name="pinned">
        </div>

        <div>
          <label for="middleName">Middle Name:</label>
          <input type="text" v-model="activeRelative.middleName" id="middleName" name="middleName">
        </div>

        <div>
          <label for="sex">Sex:</label>
          <select name="sex" id="sex" v-model="activeRelative.sex">
            <option value="male">Male</option>
            <option value="female">Female</option>
          </select>
        </div>

        <div>
          <label for="birthday">Birthday:</label>
          <input type="date" v-model="activeRelative.birthday" id="birthday" name="birthday">
        </div>
        <div>
          <label for="birthday">Died AT:</label>
          <input type="date" v-model="activeRelative.diedAt" id="birthday" name="birthday">
        </div>

        <div>
          <label for="state">State:</label>
          <input type="text" id="state" v-model="activeRelative.state" name="email">
        </div>
        <div>
          <label for="address">Addres:</label>
          <input type="text" id="address" v-model="activeRelative.address" name="email">
        </div>
        <div>
          <label for="sameness">Sameness:</label>
          <input type="number" id="sameness" v-model="activeRelative.sameness" name="sameness" step="1" min="0"
            max="10">
        </div>

        <div>
          <div v-if="activeRelative.mother">
            <label for="removeMother">Remove Mother</label>
            <input type="checkbox" v-model="removeMother" id="removeMother" name="removeMother">
          </div>
          <div>
            <label for="mother">Mother: {{ activeRelative.mother ? `${activeRelative.mother}` : '' }}</label>
            <select name="mother" id="mother" v-model="newRelative.motherId">
              <option v-for="mother in relativesStore.femaleParents" :value="mother.id" :key=mother.id>
                {{ mother.firstName + ' ' + mother.lastName }}
              </option>
            </select>
          </div>
        </div>
        <div>
          <div v-if="activeRelative.father">
            <label for="removeFather">Remove Father</label>
            <input type="checkbox" v-model="removeFather" id="removeFather" name="removeFather">
          </div>
          <div>
            <label for="father">Father: {{ activeRelative.father ? `${activeRelative.father}` : '' }}</label>
            <select name="father" id="father" v-model="newRelative.fatherId">
              <option v-for="father in relativesStore.maleParents" :value="father.id" :key="father.id">
                {{ father.firstName + ' ' + father.lastName }}
              </option>
            </select>
          </div>
        </div>

        <div>
          <label for="phone">Phone:</label>
          <input type="tel" id="phone" v-model="activeRelative.phone" name="phone">
        </div>

        <div>
          <label for="email">Email:</label>
          <input type="email" id="email" v-model="activeRelative.email" name="email">
        </div>
        <div>
          <label for="lostReason">Lost Reason:</label>
          <select name="lostReason" id="lostReason" v-model="activeRelative.lostReason">
            <option value="reason 1">Reason 1</option>
            <option value="reason 2">Reason 2</option>
            <option value="reason 3">Reason 3</option>
            <option value="reason 4">Reason 4</option>
            <option value="reason 5">Reason 5</option>
          </select>
        </div>

        <div class="female-specific" v-if="activeRelative.sex == 'female'">
          <div>
            <label for="swarthy">Swarthy:</label>
            <input type="number" id="swarthy" name="swarthy" v-model="activeRelative.swarthy" step="1" min="0" max="10">
          </div>

          <div>
            <label for="hotness">Hotness:</label>
            <input type="number" id="hotness" name="hotness" v-model="activeRelative.hotness" step="1" min="0" max="10">
          </div>

          <div>
            <label for="crazy">Crazy:</label>
            <input type="number" id="crazy" name="crazy" v-model="activeRelative.crazy" step="1" min="0" max="10">
          </div>
        </div>

        <div v-if="activeRelative.sex == 'male'">
          <div>
            <label for="employable">Employable:</label>
            <input type="number" id="employable" v-model="activeRelative.employable" name="employable" step="1" min="0"
              max="10" value="0">
          </div>
        </div>
        <button type="submit">Update</button>
      </form>
    </div>
    <div class="image__container">
      <div
        style="height: 200px; width: 200px;display: flex; justify-content: center; align-items: center; overflow: hidden;">
        <img v-if="chosenImage.length" :src="convertFileSrc(chosenImage)" alt=""
          style="max-width: 100%; max-height: 100%; object-fit: contain; width: auto; height: auto;">
      </div>
      <table>
        <thead>
          <tr>
            <th> filename </th>
          </tr>
        </thead>
        <tbody>
          <tr @click="chosenImage = image.filename" v-for="image in imageStore.images">
            <td>
              {{ image.filename.split('/').pop() }}
            </td>
          </tr>
        </tbody>
      </table>
      <button @click="uploadImage">Upload Image</button>
    </div>
    <div class="metainfo">
      <h3>Metadata Information</h3>
      <div class="info-item">
        <span class="label">ID:</span>
        <span class="value">{{ activeRelative.id }}</span>
      </div>
      <div class="info-item">
        <span class="label">Created At:</span>
        <span class="value">{{ activeRelative.createdAt }}</span>
      </div>
      <div class="info-item">
        <span class="label">Updated At:</span>
        <span class="value">{{ activeRelative.updatedAt }}</span>
      </div>
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
  height: calc(100vh- 20%);
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

.metainfo {
  background-color: #f9fafb;
  padding: 16px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  border: 1px solid #e5e7eb;

}

.metainfo h3 {
  font-size: 18px;
  font-weight: bold;
  color: #4b5563;
  margin-bottom: 16px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
  font-size: 14px;
  color: #6b7280;
}

.label {
  font-weight: 600;
  color: #4b5563;
}

.value {
  color: #111827;
}

/* Add some spacing between sections */
.info-item:last-child {
  margin-bottom: 0;
}


.btn-container {
  padding: var(--size-sm);
  display: flex;
  justify-content: end;
}

.female-specific {
  display: flex;
  flex-direction: row !important;
}

.female-specific div {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
</style>
