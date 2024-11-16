<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { reactive, ref, watchEffect, type Ref } from 'vue';
import { inject } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Modal from './Modal.vue';
import { CreateNoteParams, Note, UpdateNoteParams } from '../utils/notes';
import { CreateFileParams, File } from '../utils/file';

const showNotes = inject<Ref<Boolean>>("showNotes");
const active_relative_id = inject("active_relative_id") as Ref<number, number>
const notes: Ref<Note[], Note[]> = ref([])
const files: Ref<File[], File[]> = ref([])

watchEffect(() => {
  invoke("notes_by_relative_id", { activeRelativeId: active_relative_id.value }).then((val) => {
    console.log(val)
    notes.value = val as Array<Note>;
  }).catch(e => {
    if (e instanceof Error) {
      console.log(e.message, e.stack, e.name)
    } else {
      console.log(e)
    }
  })

  invoke("files_by_relative_id", { activeRelativeId: active_relative_id.value }).then((val) => {
    files.value = val as Array<File>
    console.log(val)
  }).catch(e => {
    if (e instanceof Error) {
      console.log(e.message, e.stack, e.name)
    } else {
      console.log(e)
    }
  })
})

const sectionToShow = ref(0)
const showAddNoteModal = ref(false)
const createNotesParams = reactive<Partial<CreateNoteParams>>({})

function createNote() {
  invoke("create_note", { params: { pinned: createNotesParams.pinned ? true : false, text: createNotesParams.text, relative_id: active_relative_id.value } }).then(() => {
    invoke("notes_by_relative_id", { activeRelativeId: active_relative_id.value }).then((val) => {
      console.log(val)
      notes.value = val as Array<Note>;
    }).catch(e => {
      if (e instanceof Error) {
        console.log(e.message, e.stack, e.name)
      } else {
        console.log(e)
      }
    })

  }).catch(e => {
    console.log(e)
  })
}

const selectedNote = ref<Partial<Note>>({})

function saveEditedNote() {
  let editedNote: UpdateNoteParams = { pinned: selectedNote.value.pinned ? true : false, text: selectedNote.value.text as string, id: selectedNote.value.id as number }
  invoke("edit_note", { editedNote }).then(() => {
    invoke("notes_by_relative_id", { activeRelativeId: active_relative_id.value }).then((val) => {
      console.log(val)
      notes.value = val as Array<Note>;
    }).catch(e => {
      if (e instanceof Error) {
        console.log(e.message, e.stack, e.name)
      } else {
        console.log(e)
      }
    })
  }).catch(e => {
    console.log(e)
  })
}


function deleteEdtitedNote(id: number) {
  invoke("delete_note", { noteId: id }).then(() => {
    invoke("notes_by_relative_id", { activeRelativeId: active_relative_id.value }).then((val) => {
      console.log(val)
      notes.value = val as Array<Note>;
    }).catch(e => {
      if (e instanceof Error) {
        console.log(e.message, e.stack, e.name)
      } else {
        console.log(e)
      }
    })
    selectedNote.value = {}
  }).catch(e => {
    console.log(e)
  })
}

function openFile() {
  open({
    multiple: false,
    directory: false,
  }).then((file) => {
    if (file) {
      let params: CreateFileParams = { filePath: file, relativeId: active_relative_id.value }
      invoke("create_file", { params }).then(() => {
        invoke("files_by_relative_id", { activeRelativeId: active_relative_id.value }).then((val) => {
          files.value = val as Array<File>
          console.log(val)
        }).catch(e => {
          if (e instanceof Error) {
            console.log(e.message, e.stack, e.name)
          } else {
            console.log(e)
          }
        })
      }).catch(e => {
        console.log(e)
      })
    }
    console.log(file);
  });
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
        <button @click="sectionToShow = 0">Notes</button>
        <button @click="sectionToShow = 1">files</button>
      </div>
      <button @click="showNotes = false">Close</button>

    </div>

    <div class="notes__container notes" v-if="sectionToShow == 0">
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
              <tr v-for="note in notes" class="note__row" @click="selectedNote = note" :key="note.id">
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
        <div v-if="selectedNote.id">
          <textarea name="edit-note" id="edit-note" v-model="selectedNote.text"></textarea>
          <button @click="selectedNote.pinned = !selectedNote.pinned">{{ selectedNote.pinned ? "Unpin" : "Pin"
            }}</button>
          <button @click="saveEditedNote">Save</button>
          <button @click="deleteEdtitedNote(selectedNote.id)">delete</button>
        </div>
      </div>
    </div>
    <div class="notes__container files" v-if="sectionToShow == 1">
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
              <tr v-for="file in files" class="note__row" :key="file.id">
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
