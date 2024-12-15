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
import { successTitle, successValue, showSuccess } from '../composables/success';

const relativesStore = useRelativesStore()
const stateStore = useStateStore()
const imageStore = useImagesStore()
const activeRelative = ref({}) as Ref<RelativeIndividual, RelativeIndividual>
const newRelative = reactive({}) as CreateRelativeParams;
const removeMother = ref(false)
const removeFather = ref(false)
const chosenImage = ref('')


import Modal from './Modal.vue';
import { CreateNoteParams } from '../utils/notes';
import { useNotesStore } from '../store/notes';
import { useFilesStore } from '../store/files';

const notesStore = useNotesStore()
const filesStore = useFilesStore()

watchEffect(() => {
  notesStore.getNotes(stateStore.activeRelativeId)
  filesStore.getFiles(stateStore.activeRelativeId)
})

let createNotesParams = reactive<Partial<CreateNoteParams>>({})

function createNote() {
  notesStore.createNote(createNotesParams, stateStore.activeRelativeId)
  createNotesParams = {}
}

function saveEditedNote() {
  notesStore.saveEditedNote(stateStore.activeRelativeId)
}

function deleteEdtitedNote(id: number) {
  notesStore.deleteNote(id, stateStore.activeRelativeId)
}

function openFile() {
  filesStore.createFile(stateStore.activeRelativeId)
}


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
    successValue.value = "Saved Relatvie"
    successTitle.value = "Saved Relative"
    showSuccess.value = true
    setTimeout(() => {
      showSuccess.value = false
    }, 700)
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
            <FormInput type="text" name="maidenName" v-model="activeRelative.maidenName" title="Maiden Name" />
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
            <FormInput type="text" name="zipcode" v-model="activeRelative.city" title="zipcode" />
            <FormInput type="text" name="city" v-model="activeRelative.city" title="City" />
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
  <Modal cancel-title="Cancel" @close-modal="notesStore.showAddNoteModal = false"
    :model_open="notesStore.showAddNoteModal">
    <div class="note-form-content"
      :class="{ 'note-form-content--light': !stateStore.darkTheme, 'note-form-content--dark': stateStore.darkTheme }">
      <form @submit.prevent="createNote">
        <div class="form-group" style="display: flex; flex-direction: column;">
          <label for="note">Note</label>
          <textarea name="note" id="note" v-model="createNotesParams.text"></textarea>
        </div>
        <button class="form-submit">Save</button>
      </form>
    </div>
  </Modal>

  <div class="notes-panel"
    :class="{ 'notes-panel--light': !stateStore.darkTheme, 'notes-panel--dark': stateStore.darkTheme }">
    <nav class="notes-nav"
      :class="{ 'notes-nav--light': !stateStore.darkTheme, 'notes-nav--dark': stateStore.darkTheme }">
      <div class="nav-tabs">
        <button :class="{
          'nav-tab': true,
          'nav-tab--light': !stateStore.darkTheme,
          'nav-tab--dark': stateStore.darkTheme,
          'nav-tab--active': notesStore.section == 0
        }" @click="notesStore.changeSection(0); filesStore.activeFileId = 0; notesStore.notesEditAreaChanged = false">
          Notes
        </button>
        <button :class="{
          'nav-tab': true,
          'nav-tab--light': !stateStore.darkTheme,
          'nav-tab--dark': stateStore.darkTheme,
          'nav-tab--active': notesStore.section == 1
        }" @click="notesStore.changeSection(1); notesStore.activeNoteId = 0; notesStore.notesEditAreaChanged = false">
          Files
        </button>
      </div>
      <div style="display: flex; gap: var(--size-sm);">
        <button v-if="notesStore.section == 1" class="action-button"
          :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
          @click="openFile">
          Add File
        </button>
        <button v-if="notesStore.section == 0" class="action-button"
          :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
          @click="notesStore.showAddNoteModal = true">
          Add Note
        </button>
      </div>

    </nav>

    <!-- Notes Section -->
    <div class="content-grid" v-if="notesStore.section == 0">
      <div class="content-list"
        :class="{ 'content-list--light': !stateStore.darkTheme, 'content-list--dark': stateStore.darkTheme }">
        <table class="data-table">
          <thead class="data-table__header" :class="{ 'data-table__header--dark': stateStore.darkTheme }">
            <tr>
              <th class="data-table__header-cell">Note</th>
              <th class="data-table__header-cell" style="width: 60px; ">Pinned</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="note in notesStore.notes" :key="note.id" class="data-table__row" :class="{
              'data-table__row--light': !stateStore.darkTheme,
              'data-table__row--dark': stateStore.darkTheme,
              'data-table__row--selected': notesStore.activeNoteId == note.id
            }" @click="notesStore.activeNote = note; notesStore.activeNoteId = note.id">
              <td class="data-table__cell">{{ note.text }}</td>
              <td class="data-table__cell" style="width: 60px; ">{{ note.pinned ? '✅' : '' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="content-preview"
        :class="{ 'content-preview--light': !stateStore.darkTheme, 'content-preview--dark': stateStore.darkTheme }">
        <div v-if="notesStore.activeNoteId > 0" class="note-editor">
          <textarea name="edit-note" id="edit-note" v-model="notesStore.activeNote.text"
            @input="notesStore.notesEditAreaChanged = true"
            :class="{ 'note-editor__textarea--light': !stateStore.darkTheme, 'note-editor__textarea--dark': stateStore.darkTheme }"></textarea>
        </div>

        <div v-if="notesStore.activeNoteId > 0" class="action-buttons">
          <button class="action-button"
            :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
            @click="notesStore.pinNote(stateStore.activeRelativeId)">
            {{ notesStore.activeNote.pinned ? "Unpin" : "Pin" }}
          </button>
          <button v-if="notesStore.notesEditAreaChanged" class="action-button"
            :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
            @click="saveEditedNote">
            Save
          </button>
          <button class="action-button action-button--delete" @click="deleteEdtitedNote(notesStore.activeNote.id)">
            Delete
          </button>
        </div>
      </div>
    </div>

    <div class="content-grid" v-if="notesStore.section == 1">
      <div class="content-list"
        :class="{ 'content-list--light': !stateStore.darkTheme, 'content-list--dark': stateStore.darkTheme }">
        <table class="data-table">
          <thead class="data-table__header" :class="{ 'data-table__header--dark': stateStore.darkTheme }">
            <tr>
              <th class="data-table__header-cell">File Name</th>
              <th class="data-table__header-cell" style="width: 80px;">File Type</th>
              <th class="data-table__header-cell" style="width: 40px; ">Pinned</th>
            </tr>
          </thead>
          <tbody class="tbody" :class="{ 'tbody--dark': stateStore.darkTheme }">
            <tr v-for="file in filesStore.files" :key="file.id" class="data-table__row" :class="{
              'data-table__row--light': !stateStore.darkTheme,
              'data-table__row--dark': stateStore.darkTheme,
              'data-table__row--selected': filesStore.activeFileId == file.id
            }" @click="filesStore.activeFile = file; filesStore.activeFileId = file.id">
              <td class="data-table__cell">{{ file.fileName }}</td>
              <td class="data-table__cell" style="width: 80px;">{{ file.type }}</td>
              <td class="data-table__cell" style="width: 40px;">{{ file.pinned ? '✅' : '' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="content-preview"
        :class="{ 'content-preview--light': !stateStore.darkTheme, 'content-preview--dark': stateStore.darkTheme }">
        <div v-if="filesStore.activeFileId > 0">
          <div>
            <h1>FileName: {{ filesStore.activeFile.fileName }}</h1>
            <h3>FileType: {{ filesStore.activeFile.type }}</h3>
          </div>
        </div>

        <div v-if="filesStore.activeFileId > 0" class="action-buttons">
          <button class="action-button"
            :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
            @click="filesStore.pinFile(stateStore.activeRelativeId)">
            {{ filesStore.activeFile.pinned ? 'Unpin' : 'Pin' }}
          </button>
          <button class="action-button action-button--delete"
            @click="filesStore.deleteFile(stateStore.activeRelativeId)">
            Delete
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Metadata container -->
  <hr class="divider">
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



.notes-panel {
  height: 320px;
  width: 100%;
  bottom: 0;
  left: 0;
  right: 0;
  border-top: 1px solid var(--clr-frost3);
  font-family: 'Arial', sans-serif;
}

.notes-panel--light {
  background: var(--clr-light);
  color: var(--clr-dark);
}

.notes-panel--dark {
  background: var(--clr-dark1);
  color: var(--clr-light);
}

.notes-nav {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.225rem;
  border-bottom: 1px solid var(--clr-frost3);
}

.notes-nav--light {
  background: var(--clr-light1);
}

.notes-nav--dark {
  background: var(--clr-dark);
}

.nav-tab {
  border: none;
  padding: 5px 10px;
  margin: 0 5px;
  border-radius: 4px;
  cursor: pointer;
}

.nav-tab--light {
  background: var(--clr-light2);
  color: var(--clr-dark);
}

.nav-tab--dark {
  background: var(--clr-dark1);
  color: var(--clr-light);
}

.nav-tab--active {
  background: var(--clr-frost);
  color: var(--clr-light);
}

.content-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  height: calc(100% - 50px);
}

.content-list {
  overflow-y: auto;
  border-right: 1px solid var(--clr-frost3);
}

.content-list--light {
  background: var(--clr-light);
}

.content-list--dark {
  background: var(--clr-dark1);
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.data-table__header {
  position: sticky;
  top: 0;
  color: var(--clr-dark) !important;
}

.data-table__header--dark {
  background: var(--clr-dark1);
}

.data-table__header-cell {
  padding: 10px;
  border-bottom: 2px solid var(--clr-frost3);
  text-align: left;
}

.data-table__row {
  cursor: pointer;
}

.data-table__row--light {
  background: var(--clr-light2);
  color: var(--clr-dark);
}

.data-table__row--dark {
  background: var(--clr-dark1);
  color: var(--clr-light);
}

.data-table__row--selected {
  background: var(--clr-frost);
  color: var(--clr-light);
}

.data-table__cell {
  padding: 4px;
  border-bottom: 1px solid var(--clr-frost3);
  padding-left: 10px;
}

.content-preview {
  padding: 8px;
}

.content-preview--light {
  background: var(--clr-light);
}

.content-preview--dark {
  background: var(--clr-dark1);
}

.action-buttons {
  display: flex;
  gap: 8px;
}

.action-button {
  margin: 5px 0;
  padding: 5px 10px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.action-button--light {
  background: var(--clr-frost2);
  color: var(--clr-light);
}

.action-button--dark {
  background: var(--clr-frost);
  color: var(--clr-light);
}

.action-button--delete {
  background: var(--clr-aurora);
  color: var(--clr-light);
}

.action-button--delete:hover {
  background: var(--clr-aurora1);
}

.note-editor {
  margin: 10px 0;
}

.note-editor__textarea--light {
  background: var(--clr-light2);
  color: var(--clr-dark);
  border: 1px solid var(--clr-frost3);
  width: 100%;
  min-height: 100px;
  padding: 8px;
  border-radius: 4px;
}

.note-editor__textarea--dark {
  background: var(--clr-dark2);
  color: var(--clr-light);
  border: 1px solid var(--clr-frost3);
  width: 100%;
  min-height: 100px;
  padding: 8px;
  border-radius: 4px;
}

/* Hover states */
.data-table__row:hover:not(.data-table__row--selected) {
  background: var(--clr-frost3);
  color: var(--clr-dark);
}

.action-button:hover:not(.action-button--delete) {
  background: var(--clr-frost2);
  color: var(--clr-light);
}

.nav-tab:hover:not(.nav-tab--active) {
  background: var(--clr-frost3);
  color: var(--clr-dark);
}


.data-table__header {
  position: sticky;
  top: 0;
}


.data-table__header--dark {
  background: var(--clr-dark2);
  color: var(--clr-light) !important;
}

.data-table__header-cell {
  padding: 10px;
  border-bottom: 2px solid var(--clr-frost3);
  text-align: left;
}

.data-table__header-cell--light {
  color: var(--clr-dark);
}

.data-table__header-cell--dark {
  color: var(--clr-light);
}


.tbody {
  background-color: var(--clr-dark2);
}


.tbody--dark {
  background-color: var(--clr-dark2);

}


.note-form-content {
  width: 400px;
  padding: 20px;
  border-radius: 8px;
  transition: all 0.3s ease;
}

.note-form-content--light {
  background-color: var(--clr-light);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.note-form-content--dark {
  background-color: var(--clr-dark1);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}


.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  font-size: 14px;
}

.note-form-content--light label {
  color: var(--clr-dark);
}

.note-form-content--dark label {
  color: var(--clr-light);
}

textarea {
  width: 100%;
  min-height: 150px;
  padding: 12px;
  border-radius: 4px;
  font-family: inherit;
  font-size: 14px;
  resize: vertical;
  transition: all 0.3s ease;
}

.note-form-content--light textarea {
  background-color: var(--clr-light);
  border: 1px solid var(--clr-light2);
  color: var(--clr-dark);
}

.note-form-content--dark textarea {
  background-color: var(--clr-dark2);
  border: 1px solid var(--clr-dark);
  color: var(--clr-light);
}

textarea:focus {
  outline: none;
}

.note-form-content--light textarea:focus {
  border-color: var(--clr-frost2);
  box-shadow: 0 0 0 2px rgba(94, 129, 172, 0.2);
}

.note-form-content--dark textarea:focus {
  border-color: var(--clr-frost3);
  box-shadow: 0 0 0 2px rgba(136, 192, 208, 0.2);
}

.form-submit {
  width: 100%;
  padding: 12px;
  border: none;
  border-radius: 4px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.note-form-content--light .form-submit {
  background-color: var(--clr-frost);
  color: var(--clr-light);
}

.note-form-content--dark .form-submit {
  background-color: var(--clr-frost3);
  color: var(--clr-dark);
}

.note-form-content--light .form-submit:hover {
  background-color: var(--clr-frost2);
}

.note-form-content--dark .form-submit:hover {
  background-color: var(--clr-frost2);
  color: var(--clr-light);
}
</style>
