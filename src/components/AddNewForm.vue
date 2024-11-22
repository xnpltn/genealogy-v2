<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import type { CreateRelativeParams } from '../utils/types';
import { useRelativesStore } from '../store/relatives';
import { useStateStore } from '../store/state';
import Modal from './Modal.vue';
import { open } from '@tauri-apps/plugin-dialog';
import UpdateForm from './UpdateForm.vue';

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
    <div class="main-container">
      <div class="form-container">
        <form class="form" @submit.prevent="save">
          <!-- names -->
          <div class="form-group" id="names">
            <div class="input-container">
              <label for="firstName" class="input-label">First Name</label>
              <input type="text" id="firstName" name="firstName" class="form-imput" v-model="newRelative.firstName">
            </div>
            <div class="input-container">
              <label for="middleName" class="input-label">Middle Name</label>
              <input type="text" id="middleName" name="middleName" class="form-imput" v-model="newRelative.middleName">
            </div>
            <div class="input-container">
              <label for="secondName" class="input-label">Last Name</label>
              <input type="text" id="secondName" name="secondName" class="form-imput" v-model="newRelative.lastName">
            </div>
            <div class="input-container">
              <label for="secondName" class="input-label">Sex</label>
              <select name="sex" id="sex" class="form-select" v-model="newRelative.sex">
                <option value="male">Male</option>
                <option value="female">Female</option>
              </select>
            </div>
            <div class="input-container">
              <label for="birthday" class="input-label">Birthday</label>
              <input name="birthday" id="birthday" class="form-input" v-model="newRelative.birthday">
            </div>
            <div class="input-container">
              <label for="birthday" class="input-label">Dies AT</label>
              <input name="diedAt" id="diedAt" class="form-input" v-model="newRelative.diedAt">
            </div>
          </div>
          <!-- contacts-->
          <div class="form-group" id="contacts">
            <div class="input-container">
              <label for="email" class="input-label">Email</label>
              <input type="text" id="email" name="email" class="form-imput" v-model="newRelative.email">
            </div>
            <div class="input-container">
              <label for="phone" class="input-label">Phone</label>
              <input type="text" id="phone" name="phone" class="form-imput" v-model="newRelative.phone">
            </div>
            <div class="input-container">
              <label for="state" class="input-label">State</label>
              <input type="text" id="state" name="state" class="form-imput" v-model="newRelative.state">
            </div>
            <div class="input-container">
              <label for="address" class="input-label">Address</label>
              <input type="text" id="address" name="address" class="form-imput" v-model="newRelative.address">
            </div>
          </div>
          <!-- parents -->
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
                <option v-for="father in relativesStore.maleParents" :value="father.id" :key="father.id">
                  {{ father.firstName + ' ' + father.lastName }}
                </option>
              </select>
            </div>
          </div>
          <!-- additional fields -->
          <div class="form-group" id="addition">
            <div class="input-container" v-if="newRelative.sex == 'male'">
              <label for="employable" class="input-label">Employable</label>
              <input type="number" id="employable" name="employable" class="form-imput"
                v-model="newRelative.employable">
            </div>
            <div class="input-container" v-if="newRelative.sex == 'female'">
              <label for="employable" class="input-label">Swarthy</label>
              <input type="number" id="swarthy" name="swarthy" class="form-imput" v-model="newRelative.swarthy">
            </div>
            <div class="input-container" v-if="newRelative.sex == 'female'">
              <label for="hotness" class="input-label">Hotness</label>
              <input type="number" id="hotness" name="hotness" class="form-imput" v-model="newRelative.hotness">
            </div>
            <div class="input-container" v-if="newRelative.sex == 'female'">
              <label for="crazy" class="input-label">Crazy</label>
              <input type="number" id="crazy" name="creazy" class="form-imput" v-model="newRelative.crazy">
            </div>
            <div class="input-container">
              <label for="lostReason" class="input-label">Lost Reason</label>
              <select name="lostReason" id="lostReason" class="form-select" v-model="newRelative.lostReason">
                <option value="1">Reason 1</option>
                <option value="2">Reason 2</option>
              </select>
            </div>
          </div>

          <div class="form-group" id="pinned">
            <div class="input-container">
              <label for="pinned" class="input-label">Sameness</label>
              <input type="number" id="sameness" name="sameness" class="form-imput" v-model="newRelative.sameness">
            </div>
            <div class="input-container">
              <label for="pinned" class="input-label">Pinned</label>
              <input type="checkbox" id="pinned" name="pinned" class="form-imput" v-model="newRelative.pinned">
            </div>
          </div>
          <div class="form-group" id="button">
            <button type="submit">Submit</button>
          </div>
        </form>
      </div>
    </div>
    <div class="image-container">
      <div class="image-section">
        <div style="height: 200px; width: 200px; background-color: red;">
          <img v-if="chosenImage.length" :src="convertFileSrc(chosenImage)" alt="">
        </div>
        <button @click="uploadImage">Upload Image</button>

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
