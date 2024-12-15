<script setup lang="ts">
import Modal from './Modal.vue';
import { reactive, watchEffect } from 'vue';
import { CreateNoteParams } from '../utils/notes';
import { useStateStore } from '../store/state';
import { useNotesStore } from '../store/notes';
import { useFilesStore } from '../store/files';

const stateStore = useStateStore()
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

function close() {
  filesStore.activeFileId = 0
  stateStore.setShowNotesToFalse()
}
</script>


<template>
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
        <button class=" nav-close action-button"
          :class="{ 'nav-close--light': !stateStore.darkTheme, 'nav-close--dark': stateStore.darkTheme }"
          @click="close">
          Close
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
</template>


<style scoped>
.nav-close--light {
  background: black;
  color: white
}


.notes-panel {
  height: 320px;
  width: 100%;
  position: absolute;
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

.form-group {
  margin-bottom: 20px;
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
