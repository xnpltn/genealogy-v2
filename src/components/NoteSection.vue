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
          <thead class="data-table__header" :class="{ 'data-table__header--dark': stateStore.darkTheme }">
            <tr>
              <th class="data-table__header-cell">File Name</th>
              <th class="data-table__header-cell">File Type</th>
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
              <td class="data-table__cell">{{ file.type }}</td>
              <td class="data-table__cell" style="width: 40px;">{{ file.pinned ? '✅' : '' }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="content-preview"
        :class="{ 'content-preview--light': !stateStore.darkTheme, 'content-preview--dark': stateStore.darkTheme }">
        <button class="action-button"
          :class="{ 'action-button--light': !stateStore.darkTheme, 'action-button--dark': stateStore.darkTheme }"
          @click="openFile">
          Add File
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
    </div>
  </div>
</template>
