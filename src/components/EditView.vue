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
    }
  })
  imageStore.loadImages(stateStore.activeRelativeId)
}


</script>

<template>
  <div>
    <button class="" @click="deleteRelative">delete</button>
  </div>
  <div class="container">
    <div class="main-container">
      <div class="form-container">
        <form class="form" @submit.prevent="saveRelative">
          <!-- names -->
          <div class="form-group" id="names">
            <div class="input-container">
              <label for="firstName" class="input-label">First Name</label>
              <input type="text" id="firstName" name="firstName" class="form-imput" v-model="activeRelative.firstName">
            </div>
            <div class="input-container">
              <label for="middleName" class="input-label">Middle Name</label>
              <input type="text" id="middleName" name="middleName" class="form-imput"
                v-model="activeRelative.middleName">
            </div>
            <div class="input-container">
              <label for="secondName" class="input-label">Last Name</label>
              <input type="text" id="secondName" name="secondName" class="form-imput" v-model="activeRelative.lastName">
            </div>
            <div class="input-container">
              <label for="secondName" class="input-label">Sex</label>
              <select name="sex" id="sex" class="form-select" v-model="activeRelative.sex">
                <option value="male">Male</option>
                <option value="female">Female</option>
              </select>
            </div>
            <div class="input-container">
              <label for="birthday" class="input-label">Birthday</label>
              <input name="birthday" id="birthday" class="form-input" v-model="activeRelative.birthday">
            </div>
            <div class="input-container">
              <label for="birthday" class="input-label">Died At</label>
              <input name="diedAt" id="diedAt" class="form-input" v-model="activeRelative.diedAt">
            </div>
          </div>
          <!-- contacts-->
          <div class="form-group" id="contacts">
            <div class="input-container">
              <label for="email" class="input-label">Email</label>
              <input type="text" id="email" name="email" class="form-imput" v-model="activeRelative.email">
            </div>
            <div class="input-container">
              <label for="phone" class="input-label">Phone</label>
              <input type="text" id="phone" name="phone" class="form-imput" v-model="activeRelative.phone">
            </div>
            <div class="input-container">
              <label for="state" class="input-label">State</label>
              <input type="text" id="state" name="state" class="form-imput" v-model="activeRelative.state">
            </div>
            <div class="input-container">
              <label for="address" class="input-label">Address</label>
              <input type="text" id="address" name="address" class="form-imput" v-model="activeRelative.address">
            </div>
          </div>
          <!-- parents -->
          <div class="form-group" id="remove-parents">
            <div class="input-container" v-if="activeRelative.mother || newRelative.fatherId">
              <label for="removeMother" class="input-label">Remove Father</label>
              <input type="checkbox" id="removeFather" name="removeFather" class="form-imput" v-model="removeFather">
            </div>
            <div class="input-container" v-if="activeRelative.father || newRelative.motherId">
              <label for="removeMother" class="input-label">Remove Mother</label>
              <input type="checkbox" id="removeMother" name="removeMother" class="form-imput" v-model="removeMother">
            </div>
          </div>
          <div class="form-group" id="parents">
            <div class="input-container">
              <label for="father" class="input-label">Father</label>
              <select name="father" id="father" class="form-select" v-model="newRelative.fatherId">
                <option v-for="father in relativesStore.maleParents" :value="father.id" :key="father.id">
                  {{ father.firstName + ' ' + father.lastName }}
                </option>
              </select>
            </div>
            <div class="input-container">
              <label for="father" class="input-label">Mother</label>
              <select name="mother" id="mother" class="form-select" v-model="newRelative.motherId">
                <option v-for="mother in relativesStore.femaleParents" :value="mother.id" :key="mother.id">
                  {{ mother.firstName + ' ' + mother.lastName }}
                </option>
              </select>
            </div>
          </div>
          <!-- additional fields -->
          <div class="form-group" id="addition">
            <div class="input-container" v-if="activeRelative.sex == 'male'">
              <label for="employable" class="input-label">Employable</label>
              <input type="text" id="employable" name="employable" class="form-imput"
                v-model="activeRelative.employable">
            </div>
            <div class="input-container" v-if="activeRelative.sex == 'female'">
              <label for="employable" class="input-label">Swarthy</label>
              <input type="text" id="swarthy" name="swarthy" class="form-imput" v-model="activeRelative.swarthy">
            </div>
            <div class="input-container" v-if="activeRelative.sex == 'female'">
              <label for="hotness" class="input-label">Hotness</label>
              <input type="text" id="hotness" name="hotness" class="form-imput" v-model="activeRelative.hotness">
            </div>
            <div class="input-container" v-if="activeRelative.sex == 'female'">
              <label for="crazy" class="input-label">Crazy</label>
              <input type="text" id="crazy" name="creazy" class="form-imput" v-model="activeRelative.crazy">
            </div>
            <div class="input-container">
              <label for="lostReason" class="input-label">Lost Reason</label>
              <select name="lostReason" id="lostReason" class="form-select" v-model="activeRelative.lostReason">
                <option value="1">Reason 1</option>
                <option value="2">Reason 2</option>
              </select>
            </div>
          </div>

          <div class="form-group" id="pinned">
            <div class="input-container">
              <label for="pinned" class="input-label">Sameness</label>
              <input type="text" id="sameness" name="sameness" class="form-imput" v-model="activeRelative.sameness">
            </div>
            <div class="input-container">
              <label for="pinned" class="input-label">Pinned</label>
              <input type="checkbox" id="pinned" name="pinned" class="form-imput" v-model="activeRelative.pinned">
            </div>
          </div>
          <div class="form-group" id="button">
            <button type="submit">Save</button>
          </div>
        </form>
      </div>
      <div class="meta-container">
        <div class="meta-card">
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
      </div>
    </div>
    <div class="image-container">
      <div class="image-section">
        <div
          style="height: 200px; width: 200px;display: flex; justify-content: center; align-items: center; overflow: hidden;">
          <img v-if="chosenImage.length" :src="convertFileSrc(chosenImage)" alt=""
            style="max-width: 100%; max-height: 100%; object-fit: contain; width: auto; height: auto;">
        </div>
        <button @click="uploadImage">Upload Image</button>
      </div>
      <div class="table-section">
        table
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

      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  padding: var(--size-sm);
  display: grid;
  grid-template-columns: 2fr 1fr;
}
</style>
