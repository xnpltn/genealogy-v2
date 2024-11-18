import { defineStore } from "pinia";
import { Note, UpdateNoteParams } from "../utils/notes";
import { ref } from "vue";
import { type Ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { CreateNoteParams } from "../utils/notes";

export const useNotesStore = defineStore('notes', () => {
  const notes: Ref<Note[], Note[]> = ref([])
  const section = ref(0)
  const showAddNoteModal = ref(false)
  const activeNote = ref({}) as Ref<Note, Note>
  const activeNoteId = ref(0)

  function changeSection(id: number) {
    section.value = id
  }

  function getNotes(id: number) {
    invoke("notes_by_relative_id", { activeRelativeId: id }).then((val) => {
      console.log(val)
      notes.value = val as Array<Note>;
    }).catch(e => {
      if (e instanceof Error) {
        console.log(e.message, e.stack, e.name)
      } else {
        console.log(e)
      }
    })
  }
  function createNote(params: Partial<CreateNoteParams>, relative_id: number) {
    invoke("create_note", { params: { pinned: params.pinned ? true : false, text: params.text, relative_id: relative_id } })
      .then(() => {
        activeNoteId.value = 0
        getNotes(relative_id)
      }).catch(e => {
        console.log(e)
      })
  }
  function saveEditedNote(relative_id: number) {
    let editedNote: UpdateNoteParams = { pinned: activeNote.value.pinned ? true : false, text: activeNote.value.text as string, id: activeNote.value.id as number }
    invoke("update_note", { editedNote }).then(() => {
      activeNoteId.value = 0
      getNotes(relative_id)
    }).catch(e => {
      console.log(e)
    })
  }

  function deleteNote(note_id: number, relative_id: number) {
    invoke("delete_note", { noteId: note_id }).then(() => {
      activeNoteId.value = 0
      getNotes(relative_id)
    }).catch(e => {
      console.log(e)
    })
  }
  return {
    notes,
    section,
    showAddNoteModal,
    changeSection,
    getNotes,
    createNote,
    activeNote,
    saveEditedNote,
    deleteNote,
    activeNoteId
  }
})
