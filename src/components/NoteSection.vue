<script setup lang="ts">
import { reactive, ref, watchEffect } from 'vue';
import Modal from './Modal.vue';
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
  <Modal :model_open="showAddNoteModal" @close-modal="showAddNoteModal = false">
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
  </Modal>
  <div class="notes-sect">
    <div class="tabbar">

      <div>
        <button @click="notesStore.changeSection(0); filesStore.activeFileId = 0">Notes</button>
        <button @click="notesStore.changeSection(1); notesStore.activeNoteId = 0">files</button>
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
          <textarea name="edit-note" id="edit-note" v-model="notesStore.activeNote.text"></textarea>
          <button @click="notesStore.activeNote.pinned = !notesStore.activeNote.pinned">{{ notesStore.activeNote.pinned
            ? "Unpin" : "Pin"
            }}</button>
          <button @click="saveEditedNote">Save</button>
          <button @click="deleteEdtitedNote(notesStore.activeNote.id)">delete</button>
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
        <div v-if="filesStore.activeFileId > 0">
          <button @click="filesStore.pinFile(stateStore.activeRelativeId)">pin</button>
          <button @click="filesStore.deleteFile(stateStore.activeRelativeId)">delete</button>
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
  background: #007bff;
  color: white;
  border: none;
  padding: 5px 10px;
  margin: 0 5px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
}

.tabbar button:hover {
  background: #0056b3;
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
</style>
