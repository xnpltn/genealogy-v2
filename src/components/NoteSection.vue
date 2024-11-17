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
        <button @click="notesStore.changeSection(0)">Notes</button>
        <button @click="notesStore.changeSection(1)">files</button>
      </div>
      <button @click="stateStore.setShowNotesToFalse()">Close</button>
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
              <tr v-for="note in notesStore.notes" class="note__row" @click="notesStore.activeNote = note"
                :key="note.id">
                <td>{{ note.text }}</td>
                <td>{{ note.pinned }}</td>
              </tr>
            </tbody>
          </table>

        </div>
      </div>
      <div class="preview">
        preview notes
        <button @click="showAddNoteModal = true">add note</button>
        <div v-if="notesStore.activeNote.id">
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
              </tr>
            </thead>
            <tbody>
              <tr v-for="file in filesStore.files" class="note__row" :key="file.id">
                <td>{{ file.fileName }}</td>
                <td>{{ file.type }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="preview">
        <button @click="openFile">AddFile</button>
        preview files
      </div>
    </div>
  </div>
</template>


<style scoped>
.notes-sect {
  height: 300px;
  width: 100vw;
  background: red;
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
}

.tabbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.note__row:hover {
  cursor: pointer;
}

.notes__container {
  display: grid;
  grid-template-columns: 1fr 1fr;
  margin-top: var(--size-xl);
  height: 100%;
}

.tables {
  background-color: green;
}

.preview {
  background-color: yellow;
}
</style>
