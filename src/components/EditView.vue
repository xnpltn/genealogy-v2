<script setup lang="ts">

import { onMounted, type Ref, ref, reactive, watchEffect } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { type RelativeIndividual, type CreateRelativeParams } from '../utils/types';
import { confirm, open } from '@tauri-apps/plugin-dialog';
import { convertFileSrc } from '@tauri-apps/api/core';
import { useStateStore } from '../store/state';
import { useRelativesStore } from '../store/relatives';
import { useImagesStore } from '../store/images';
import { errorTitle, errorValue, showError } from '../composables/error';
import { convertDateToMMDDYY } from '../composables/date'
import FormInput from './form/FormInput.vue';
import FormSelect from './form/FormSelect.vue';

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
  imageStore.loadImages(stateStore.activeRelativeId)
})

watchEffect(() => {
  newRelative.sex = activeRelative.value.sex
})


onMounted(() => {
  console.log(stateStore.activeRelativeId)
  invoke("relative_by_id", { id: stateStore.activeRelativeId }).then((val) => {
    activeRelative.value = val as RelativeIndividual
    activeRelative.value.birthday = convertDateToMMDDYY(activeRelative.value.birthday)
    activeRelative.value.diedAt = convertDateToMMDDYY(activeRelative.value.diedAt)
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
    showError.value = true
    errorTitle.value = "Error Saving Relative"
    errorValue.value = `${e}`
    console.log(e)
  })
}

function deleteRelative() {
  confirm("This action is irreversible, are you sure?", { kind: 'warning' }).then(y => {
    if (y) {
      invoke("delete_relative", { relativeId: stateStore.activeRelativeId }).then(() => {
        stateStore.activeTab = 0
      }).catch(e => {
        if (`${e}`.toLowerCase().includes("invalid args")) {
          errorValue.value = "Error Occured while Updating relative. tip: Check if All required fields are present. (first and last names)"
        } else {
          errorValue.value = `${e}`
        }
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
  <div class="container" :class="{ 'container_black': stateStore.darkTheme }">
    <div class="main-container" :class="{ 'main-container_black': stateStore.darkTheme }">
      <div class="form-container" :class="{ 'form-container_black ': stateStore.darkTheme }">
        <form class="form" @submit.prevent="saveRelative">
          <!-- names -->
          <div class="form-group" id="names">
            <FormInput type="text" name="firstName" v-model="activeRelative.firstName" title="First Name" />
            <FormInput type="text" name="middleName" v-model="activeRelative.middleName" title="Middle Name" />
            <FormInput type="text" name="secondName" v-model="activeRelative.lastName" title="Last Name" />
            <FormSelect :options="[{ value: 'male', name: 'Male' }, { value: 'female', name: 'Female' }]" label="Sex"
              name="sex" v-model="activeRelative.sex" />
            <FormInput type="text" name="birthday" v-model="activeRelative.birthday" title="Birthday" />
            <FormInput type="text" name="diedAt" v-model="activeRelative.diedAt" title="Died" />
          </div>

          <!-- contacts -->
          <div class="form-group" id="contacts">
            <FormInput type="text" name="email" v-model="activeRelative.email" title="Email" />
            <FormInput type="text" name="phone" v-model="activeRelative.phone" title="Phone" />
            <FormInput type="text" name="state" v-model="activeRelative.state" title="State" />
            <FormInput type="text" name="address" v-model="activeRelative.address" title="Address" />
            <FormInput type="text" name="city" v-model="activeRelative.city" title="City" />
            <FormInput type="text" name="zipcode" v-model="activeRelative.city" title="zipcode" />
          </div>

          <!-- parents -->
          <div class="form-group" id="remove-parents">
            <FormInput v-if="activeRelative.father || newRelative.fatherId" typ="checkbox" name="removeFather"
              v-model="removeFather"
              :title="activeRelative.father ? String('Remove Father (' + activeRelative.father + ')') : 'Remove Father'" />
            <FormInput v-if="activeRelative.mother || newRelative.motherId" typ="checkbox" name="removeMother"
              v-model="removeMother"
              :title="activeRelative.father ? String('Remove Mother (' + activeRelative.mother + ')') : 'Remove Mother'" />
          </div>
          <div class="form-group" id="parents">
            <FormSelect
              :options="relativesStore.maleParents.map(father => ({ value: father.id, name: father.firstName + ' ' + father.lastName }))"
              label="Father" name="father" v-model="newRelative.fatherId" />
            <FormSelect
              :options="relativesStore.femaleParents.map(mother => ({ value: mother.id, name: mother.firstName + ' ' + mother.lastName, }))"
              label="Mother" name="mother" v-model="newRelative.motherId" />
          </div>

          <!-- additional fields -->
          <div class="form-group" id="addition">
            <FormInput v-if="activeRelative.sex == 'male'" typ="number" name="employable"
              v-model="activeRelative.employable" title="Employable" :min="0" :max="10" :step="1" />
            <FormInput v-if="activeRelative.sex == 'female'" typ="number" name="swarthy"
              v-model="activeRelative.swarthy" title="Swarthy" :min="0" :max="10" :step="1" />
            <FormInput v-if="activeRelative.sex == 'female'" typ="number" name="hotness"
              v-model="activeRelative.hotness" title="Hotness" :min="0" :max="10" :step="1" />
            <FormInput v-if="activeRelative.sex == 'female'" typ="number" name="crazy" v-model="activeRelative.crazy"
              title="Crazy" :min="0" :max="10" :step="1" />
            <FormSelect :options="relativesStore.lostReasons.map(reason => ({ value: reason, name: reason }))"
              label="Lost Reason" name="lostReason" v-model="activeRelative.lostReason" />
          </div>

          <div class="form-group last" id="pinned">
            <FormInput type="text" name="sameness" v-model="activeRelative.sameness" title="Sameness" />
            <FormInput typ="checkbox" name="pinned" v-model="activeRelative.pinned" title="Pinned" />
          </div>
          <div class="form-group last" id="button">
            <button type="submit" class="submit-button"
              :class="{ 'submit-button_black': stateStore.darkTheme }">Save</button>
          </div>
        </form>
      </div>
      <div class="meta-container">
        <div class="meta-card">
          <div class="metainfo" :class="{ 'metainfo__black': stateStore.darkTheme }">
            <h3 class="metainfo__header">Metadata Information</h3>
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
            <hr class="divider">
            <div>
              <button class="delete-button" @click="deleteRelative">delete</button>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="image-container">
      <div class="image-section">
        <div
          style="height: 300px; width: 300px;display: flex; justify-content: center; align-items: center; overflow: hidden;">
          <img v-if="chosenImage.length || imageStore.images.length"
            :src="chosenImage.length ? convertFileSrc(chosenImage) : convertFileSrc(imageStore.images[0].filename)"
            alt="" style="max-width: 100%; max-height: 100%; object-fit: contain; width: auto; height: auto;">
        </div>
        <div style="display: flex; gap: var(--size-sm);">
          <button class="action-button action-button--new action-button--light"
            :class="{ 'action-button--dark': stateStore.darkTheme }" @click="uploadImage">Upload Image</button>
          <button v-if="imageStore.activeImageId > 0"
            @click="imageStore.deleteImage(imageStore.activeImageId, stateStore.activeRelativeId); chosenImage = '';"
            class="delete-button">Delete</button>
        </div>
      </div>
      <div class="table-section" v-if="imageStore.images.length">
        <table :class="{
          'images-table': true,
          'images-table--light': !stateStore.darkTheme,
          'images-table--dark': stateStore.darkTheme
        }">
          <thead :class="{
            'data-grid__header': true,
            'data-grid__header--light': !stateStore.darkTheme,
            'data-grid__header--dark': stateStore.darkTheme
          }">
            <tr>
              <th :class="{
                'data-grid__header-cell': true,
                'data-grid__header-cell--light': !stateStore.darkTheme,
                'data-grid__header-cell--dark': stateStore.darkTheme
              }">
                filename
              </th>
            </tr>
          </thead>
          <tbody>
            <tr
              @click="chosenImage = image.filename; imageStore.pinImage(image.id); imageStore.activeImageId = image.id"
              v-for="image in imageStore.images" :key="image.id" :class="{
                'data-grid__row': true,
                'data-grid__row--light': !stateStore.darkTheme,
                'data-grid__row--dark': stateStore.darkTheme
              }">
              <td :class="{
                'data-grid__cell': true,
                'data-grid__cell--light': !stateStore.darkTheme,
                'data-grid__cell--dark': stateStore.darkTheme
              }">
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

.delete-button {
  background-color: red;
  border: none;
  border-radius: var(--size-xss);
  color: var(--clr-light);
  padding-top: 5px;
  padding-left: var(--size-sm);
  padding-right: var(--size-sm);
  padding-bottom: 5px;
  border-radius: 5px;
}

.delete-button:hover {
  font-weight: bold;
}

.metainfo__black {
  color: var(--clr-light);
}

.metainfo__header {
  margin-bottom: 5px;
  margin-top: 1px;
}
</style>
