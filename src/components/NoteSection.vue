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

const createNotesParams = reactive<Partial<CreateNoteParams>>({})

function createNote() {
  notesStore.createNote(createNotesParams, stateStore.activeRelativeId)
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
  <Modal @close-modal="notesStore.showAddNoteModal = false" :model_open="notesStore.showAddNoteModal">
    <div class="note-form-header"
      :class="{ 'note-form-header--light': !stateStore.darkTheme, 'note-form-header--dark': stateStore.darkTheme }">
    </div>
    <div class="note-form-content"
      :class="{ 'note-form-content--light': !stateStore.darkTheme, 'note-form-content--dark': stateStore.darkTheme }">
      <form @submit.prevent="createNote">
        <div class="form-group">
          <label for="note">Note</label>
          <textarea name="note" id="note" v-model="createNotesParams.text"></textarea>
        </div>
        <div class="form-group">
          <label for="pinned">Pinned</label>
          <input type="checkbox" v-model="createNotesParams.pinned">
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
      <button class="nav-close"
        :class="{ 'nav-close--light': !stateStore.darkTheme, 'nav-close--dark': stateStore.darkTheme }" @click="close">
        Close
      </button>
    </nav>

    <!-- Notes Section -->
    <div class="content-grid" v-if="notesStore.section == 0">
      <div class="content-list"
        :class="{ 'content-list--light': !stateStore.darkTheme, 'content-list--dark': stateStore.darkTheme }">
        <table class="data-table">
          <thead class="data-table__header">
            <tr>
              <th>Note</th>
              <th>Pinned</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="note in notesStore.notes" :key="note.id" class="data-table__row" :class="{
              'data-table__row--light': !stateStore.darkTheme,
              'data-table__row--dark': stateStore.darkTheme,
              'data-table__row--selected': notesStore.activeNoteId == note.id
            }" @click="notesStore.activeNote = note; notesStore.activeNoteId = note.id">
              <td>{{ note.text }}</td>
              <td>{{ note.pinned }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="content-preview"
        :class="{ 'content-preview--light': !stateStore.darkTheme, 'content-preview--dark': stateStore.darkTheme }">
        <button class="action-button"
          :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
          @click="notesStore.showAddNoteModal = true">
          Add Note
        </button>

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
          <thead class="data-table__header">
            <tr>
              <th>file name</th>
              <th>file type</th>
              <th>Pinned</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="file in filesStore.files" :key="file.id" class="data-table__row" :class="{
              'data-table__row--light': !stateStore.darkTheme,
              'data-table__row--dark': stateStore.darkTheme,
              'data-table__row--selected': filesStore.activeFileId == file.id
            }" @click="filesStore.activeFile = file; filesStore.activeFileId = file.id">
              <td>{{ file.fileName }}</td>
              <td>{{ file.type }}</td>
              <td>{{ file.pinned }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="content-preview"
        :class="{ 'content-preview--light': !stateStore.darkTheme, 'content-preview--dark': stateStore.darkTheme }">
        <button class="action-button"
          :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
          @click="openFile">
          Add Note
        </button>

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
      <!-- ... rest of the template -->
    </div>
  </div>
</template>


<style scoped>
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
  background: var(--clr-light2);
}

.notes-panel--dark {
  background: var(--clr-dark2);
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
  background: var(--clr-dark1);
}

.nav-tab {
  border: none;
  padding: 5px 10px;
  margin: 0 5px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.nav-tab--light {
  background: var(--clr-light2);
  color: var(--clr-dark);
}

.nav-tab--dark {
  background: var(--clr-dark2);
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
  background: var(--clr-dark);
}

.data-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.data-table__header {
  position: sticky;
  top: 0;
}

.data-table__header--light {
  background: var(--clr-light1);
}

.data-table__header--dark {
  background: var(--clr-dark1);
}

.data-table__row {
  cursor: pointer;
  transition: background-color 0.2s;
}

.data-table__row--light {
  background: var(--clr-light);
  color: var(--clr-dark);
}

.data-table__row--dark {
  background: var(--clr-dark);
  color: var(--clr-light);
}

.data-table__row--selected {
  background: var(--clr-frost);
  color: var(--clr-light);
}

.action-button {
  margin: 5px 0;
  padding: 5px 10px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.action-button--light {
  background: var(--clr-frost3);
  color: var(--clr-dark);
}

.action-button--dark {
  background: var(--clr-frost);
  color: var(--clr-light);
}

.action-button--delete {
  background: var(--clr-aurora);
  color: var(--clr-light);
}

.note-editor__textarea {
  width: 100%;
  min-height: 100px;
  margin: 10px 0;
  padding: 5px;
  border-radius: 4px;
}

.note-editor__textarea--light {
  background: var(--clr-light);
  color: var(--clr-dark);
}

.note-editor__textarea--dark {
  background: var(--clr-dark2);
  color: var(--clr-light);
}
</style>
