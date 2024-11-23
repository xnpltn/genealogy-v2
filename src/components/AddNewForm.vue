<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue';
import { convertFileSrc, invoke } from '@tauri-apps/api/core';
import type { CreateRelativeParams } from '../utils/types';
import { useRelativesStore } from '../store/relatives';
import { useStateStore } from '../store/state';
import { useImagesStore } from '../store/images';
import { open } from '@tauri-apps/plugin-dialog';
import { errorTitle, errorValue, showError } from '../composables/error';

import FormInput from './form/FormInput.vue'
import FormSelect from './form/FormSelect.vue';

const relativesStore = useRelativesStore()
const stateStore = useStateStore()
const imagesStore = useImagesStore()
const chosenImage = ref("")

onMounted(() => {
  relativesStore.fetchMaleParents(stateStore.activeRelativeId)
  relativesStore.fetchFemaleParants(stateStore.activeRelativeId)
})

const newRelative = reactive({ sameness: 0, hotness: 0, crazy: 0, employable: 0, swarthy: 0, pinned: false, sex: 'male' }) as CreateRelativeParams;
function save() {
  let re = newRelative as CreateRelativeParams

  invoke("create_relative", { newRelative: re }).then((rel) => {
    stateStore.activeTab = 0
    let relative = rel as { id: number }
    if (chosenImage.value.length && relative.id > 0) {
      imagesStore.createImage(chosenImage.value, relative.id)
    }
  }).catch(e => {
    errorValue.value = `${e}`
    showError.value = true
    errorTitle.value = "Error Creating Relative"
    console.log(e)
  })
}

function uploadImage() {
  open({ multiple: false, directory: false, filters: [{ name: 'Image', extensions: ['jpg', 'png', 'jpeg', 'webp', 'ico'] }] }).then((file) => {
    if (file) {
      chosenImage.value = file
    }
  })
}
</script>
<template>
  <div class="container" :class="{ 'container_black': stateStore.darkTheme }">
    <div class="main-container" :class="{ 'main-container_black': stateStore.darkTheme }">
      <div class="form-container" :class="{ 'form-container_black': stateStore.darkTheme }">
        <form class="form" @submit.prevent="save">
          <!-- names -->
          <div class="form-group" id="names">
            <FormInput title="First Name" typ="text" name="firstName" v-model="newRelative.firstName" />
            <FormInput title="Middle Name" typ="text" name="middleName" v-model="newRelative.middleName" />
            <FormInput title="Last Name" typ="text" name="lastName" v-model="newRelative.lastName" />
            <FormSelect v-model="newRelative.sex" name="sex" label="Sex"
              :options="[{ value: 'male', name: 'Male' }, { value: 'female', name: 'Female' }]" />
            <FormInput title="Birthday" typ="text" name="birthday" v-model="newRelative.birthday" />
            <FormInput title="Died At" typ="text" name="diedAt" v-model="newRelative.diedAt" />
          </div>

          <!-- contacts -->
          <div class="form-group" id="contacts">
            <FormInput title="Email" typ="text" name="email" v-model="newRelative.email" />
            <FormInput title="Phone" typ="text" name="phone" v-model="newRelative.phone" />
            <FormInput title="State" typ="text" name="state" v-model="newRelative.state" />
            <FormInput title="Address" typ="text" name="address" v-model="newRelative.address" />
          </div>

          <!-- parents -->
          <div class="form-group" id="parents">
            <FormSelect v-model="newRelative.fatherId" name="father" label="Father"
              :options="relativesStore.maleParents.map(father => ({ value: father.id, name: `${father.firstName} ${father.lastName}` }))" />
            <FormSelect v-model="newRelative.motherId" name="mother" label="Mother"
              :options="relativesStore.femaleParents.map(mother => ({ value: mother.id, name: `${mother.firstName} ${mother.lastName}` }))" />
          </div>

          <div class="form-group" id="addition">
            <FormInput title="Employable" typ="number" name="employable" v-model="newRelative.employable"
              v-if="newRelative.sex === 'male'" />
            <FormInput title="Swarthy" typ="number" name="swarthy" v-model="newRelative.swarthy"
              v-if="newRelative.sex === 'female'" />
            <FormInput title="Hotness" typ="number" name="hotness" v-model="newRelative.hotness"
              v-if="newRelative.sex === 'female'" />
            <FormInput title="Crazy" typ="number" name="crazy" v-model="newRelative.crazy"
              v-if="newRelative.sex === 'female'" />
            <FormSelect v-model="newRelative.lostReason" name="lostReason" label="Lost Reason"
              :options="[{ value: '1', name: 'Reason 1' }, { value: '2', name: 'Reason 2' }]" />
          </div>

          <div class="form-group" id="pinned">
            <FormInput title="Sameness" typ="number" name="sameness" v-model="newRelative.sameness" />
            <FormInput title="Pinned" typ="checkbox" name="pinned" v-model="newRelative.pinned" />
          </div>

          <div class="form-group" id="button">
            <button class="submit-button" :class="{ 'submit-button_black': stateStore.darkTheme }" type="submit">
              Submit
            </button>
          </div>
        </form>
      </div>
    </div>
    <div class="image-container">
      <div class="image-section">
        <div style="height: 300px; width: 300px; border: 1px solid grey; border-radius: 5px;">
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
