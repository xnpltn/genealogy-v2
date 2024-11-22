<script setup lang="ts">
import Modal from './Modal.vue';
import { reactive, ref, watchEffect } from 'vue';
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

const showAddNoteModal = ref(false)
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
  <Modal @close-modal="showAddNoteModal = false" :model_open="showAddNoteModal">
    <div class="modal__tabbar">
    </div>
    <div class="modal__form-container">
      <form @submit.prevent="createNote">
        <div>
          <label for="note">Note</label>
          <textarea name="note" id="note" v-model="createNotesParams.text"></textarea>
        </div>
        <div>
          <label for="pinned">Pinned</label>
          <input type="checkbox" v-model="createNotesParams.pinned">
        </div>
        <button>submit</button>
      </form>
    </div>
  </Modal>
  <div class="notes-sect">
    <div class="tabbar">
      <div>
        <button :class="{ 'selected__section': notesStore.section == 0 }"
          @click="notesStore.changeSection(0); filesStore.activeFileId = 0; notesStore.notesEditAreaChanged = false">Notes</button>
        <button :class="{ 'selected__section': notesStore.section == 1 }"
          @click="notesStore.changeSection(1); notesStore.activeNoteId = 0; notesStore.notesEditAreaChanged = false">files</button>
      </div>
      <button @click="close">Close</button>
    </div>

    <div class="notes__container notes" v-if="notesStore.section == 0">
      <div class="tables">
        <div class="notes">
          <table>
            <thead>
              <tr>
                <th>note</th>
                <th>pinned</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="note in notesStore.notes" class="note__row"
                @click="notesStore.activeNote = note; notesStore.activeNoteId = note.id" :key="note.id"
                :class="{ 'selected': notesStore.activeNoteId == note.id }">
                <td>{{ note.text }}</td>
                <td>{{ note.pinned }}</td>
              </tr>
            </tbody>
          </table>

        </div>
      </div>
      <div class="preview">
        <button @click="showAddNoteModal = true">add note</button>
        <div v-if="notesStore.activeNoteId > 0">
          <textarea name="edit-note" id="edit-note" v-model="notesStore.activeNote.text"
            @input="notesStore.notesEditAreaChanged = true"></textarea>
        </div>
        <div v-if="notesStore.activeNoteId > 0" class="button_container">
          <button @click="notesStore.pinNote(stateStore.activeRelativeId)">
            {{ notesStore.activeNote.pinned ? "unpin" : "pin" }}
          </button>
          <button v-if="notesStore.notesEditAreaChanged" @click="saveEditedNote">Save</button>
          <button class="delete_button" @click="deleteEdtitedNote(notesStore.activeNote.id)">delete</button>
        </div>
      </div>
    </div>
    <div class="notes__container files" v-if="notesStore.section == 1">
      <div class="tables">
        <div class="files">
          <table>
            <thead>
              <tr>
                <th>file name</th>
                <th>file type</th>
                <th>Pinned</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="file in filesStore.files"
                @click="filesStore.activeFile = file; filesStore.activeFileId = file.id" class="note__row"
                :class="{ 'selected': filesStore.activeFileId == file.id }">
                <td>{{ file.fileName }}</td>
                <td>{{ file.type }}</td>
                <td>{{ file.pinned }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="preview">
        <button @click="openFile">AddFile</button>
        <div v-if="filesStore.activeFileId > 0" class="file-info-container">
          <div class="metainfo">
            <h1 class="file-name">FileName: {{ filesStore.activeFile.fileName }}</h1>
            <h3 class="file-type">FileType: {{ filesStore.activeFile.type }}</h3>
          </div>
          <div class="button-container">
            <button class="pin-button" @click="filesStore.pinFile(stateStore.activeRelativeId)">
              {{ filesStore.activeFile.pinned ? 'Unpin' : 'Pin' }}
            </button>
            <button class="delete-button" @click="filesStore.deleteFile(stateStore.activeRelativeId)">Delete</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>


<style scoped>
.notes-sect {
  height: 320px;
  width: 100%;
  background: #f4f4f4;
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  border-top: 1px solid #ddd;
  font-family: 'Arial', sans-serif;
}

.tabbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.225rem;
  background: #e9ecef;
  border-bottom: 1px solid #dee2e6;
}

.tabbar button {
  background: #495057;
  color: white;
  border: none;
  padding: 5px 10px;
  margin: 0 5px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.tabbar button:hover {
  background: #495057;
}

.notes__container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  height: calc(100% - 50px);
}

.tables {
  background: white;
  overflow-y: auto;
  border-right: 1px solid #ddd;
}

.preview {
  background: #f8f9fa;
  padding: 15px;
  display: flex;
  flex-direction: column;
}

table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

thead {
  position: sticky;
  top: 0;
  background: #f1f3f5;
}

th,
td {
  border: 1px solid #dee2e6;
  padding: 8px;
  text-align: left;
}

.note__row {
  cursor: pointer;
  transition: background-color 0.2s;
}

.note__row:hover {
  background-color: #e9ecef;
}

textarea {
  width: 100%;
  min-height: 100px;
  margin: 10px 0;
  padding: 5px;
}

.preview button {
  margin: 5px 0;
  padding: 5px 10px;
  background: #28a745;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.preview button:hover {
  background: #218838;
}

.selected {
  background: #218838;

}

.button_container {
  display: flex;
  align-items: center;
  gap: var(--size-sm);
}

.selected__section {
  background-color: #007bff !important;
}

.modal {
  z-index: 100;
  position: absolute;
  background: rgba(0, 0, 0, 0.1);
  height: 100vh;
  width: 100vw;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  top: 0;
  left: 0;

}

.modal__container {
  height: 250px;
  width: 700px;
  background: white;
  border: none;
  border-radius: var(--size-sm);
  padding: var(--size-sm);
}

.modal__tabbar {
  background-color: red;
  display: flex;
  justify-content: end;
}

.modal__form-container {
  width: 100%;
}

.metainfo {
  display: flex;
  align-items: center;
  flex-direction: column;
  gap: var(--size-sm);

}

.metainfo h1 {
  margin: 0;
  font-size: 16px;
}

.metainfo h2 {
  margin: 0;
  font-size: 14px;
}

.metainfo h3 {
  margin: 0;
  font-size: 12px;
}

.file-info-container {
  padding: 1.5rem;
  border: 1px solid #ccc;
  border-radius: 8px;
  background-color: #f9f9f9;
  margin-bottom: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.metainfo {
  margin-bottom: 1rem;
}

.file-name {
  font-size: 1.5rem;
  font-weight: bold;
  margin-bottom: 0.5rem;
  color: #333;
}

.file-type {
  font-size: 1.2rem;
  color: #666;
}

.button-container {
  display: flex;
  gap: 1rem;
}
</style>
