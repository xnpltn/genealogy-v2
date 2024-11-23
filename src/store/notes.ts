import { defineStore } from "pinia";
import { type Note, type UpdateNoteParams } from "../utils/notes";
import { ref } from "vue";
import { type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { type CreateNoteParams } from "../utils/notes";
import { confirm } from "@tauri-apps/plugin-dialog";
import { errorValue, errorTitle, showError } from "../composables/error";

export const useNotesStore = defineStore('notes', () => {
  const notes: Ref<Note[], Note[]> = ref([])
  const section = ref(0)
  const showAddNoteModal = ref(false)
  const activeNote = ref({}) as Ref<Note, Note>
  const activeNoteId = ref(0)
  const notesEditAreaChanged = ref(false)

  function changeSection(id: number) {
    section.value = id
  }

  function getNotes(id: number) {
    invoke("notes_by_relative_id", { activeRelativeId: id }).then((val) => {
      notes.value = val as Array<Note>;
    }).catch(e => {
      showError.value = true
      errorTitle.value = "Error Getting Note"
      errorValue.value = `${e}`
    })
  }
  function createNote(params: Partial<CreateNoteParams>, relative_id: number) {
    invoke("create_note", { params: { pinned: params.pinned ? true : false, text: params.text, relative_id: relative_id } })
      .then(() => {
        activeNoteId.value = 0
        showAddNoteModal.value = false
        console.log("created note")
        getNotes(relative_id)
      }).catch(e => {
        showError.value = true
        errorTitle.value = "Error  Saving Note"
        errorValue.value = `${e}`
      })
  }
  function saveEditedNote(relative_id: number) {
    let editedNote: UpdateNoteParams = { pinned: activeNote.value.pinned ? true : false, text: activeNote.value.text as string, id: activeNote.value.id as number }
    invoke("update_note", { editedNote }).then(() => {
      activeNoteId.value = 0
      getNotes(relative_id)
    }).catch(e => {
      showError.value = true
      errorTitle.value = "Error  Saving Note"
      errorValue.value = `${e}`
    })
  }

  function deleteNote(note_id: number, relative_id: number) {
    confirm('This action cannot be reverted. Are you sure?', { kind: 'warning' }).then(y => {
      if (y) {
        invoke("delete_note", { noteId: note_id }).then(() => {
          activeNoteId.value = 0
          getNotes(relative_id)
        }).catch(e => {
          showError.value = true
          errorTitle.value = "Error  Deleting Note"
          errorValue.value = `${e}`
        })
      }
    }).catch(e => {
      console.log(e)
    })
  }

  function pinNote(relative_id: number) {
    if (activeNote.value.pinned) {
      invoke('unpin_note', { noteId: activeNoteId.value }).then(() => {
        getNotes(relative_id)
        activeNoteId.value = 0
      }).catch(e => {
        console.log(e)
      })
    } else {
      invoke('pin_note', { noteId: activeNoteId.value }).then(() => {
        getNotes(relative_id)
        activeNoteId.value = 0
      }).catch(e => {
        console.log(e)
      })
    }
  }
  return {
    notes,
    section,
    notesEditAreaChanged,
    showAddNoteModal,
    changeSection,
    getNotes,
    createNote,
    activeNote,
    saveEditedNote,
    deleteNote,
    activeNoteId,
    pinNote
  }
})
