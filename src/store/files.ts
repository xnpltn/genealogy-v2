import { defineStore } from "pinia";
import { type Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { type File } from "../utils/file";
import { open, confirm } from '@tauri-apps/plugin-dialog';
import { type CreateFileParams } from "../utils/file";
import { errorValue, errorTitle, showError } from "../composables/error";



export const useFilesStore = defineStore('files', () => {
  const files: Ref<File[], File[]> = ref([])
  const activeFileId = ref(0)
  const activeFile = ref({}) as Ref<File, File>
  function getFiles(id: number) {
    invoke("files_by_relative_id", { activeRelativeId: id }).then((val) => {
      files.value = val as Array<File>
    }).catch(e => {
      showError.value = true
      errorTitle.value = "Error Getting Files"
      errorValue.value = `${e}`
    })
  }

  function createFile(id: number) {
    open({ multiple: false, directory: false }).then((file) => {
      if (file) {
        let params: CreateFileParams = { filePath: file, relativeId: id }
        invoke("create_file", { params }).then(() => {
          getFiles(id)
        }).catch(e => {
          showError.value = true
          errorTitle.value = "Error Creating Files"
          errorValue.value = `${e}`
        })
      }
      console.log(file);
    })
  }

  function deleteFile(relative_id: number) {
    confirm('This action cannot be reverted. Are you sure?', { kind: 'warning' }).then(y => {
      if (y) {
        invoke('delete_file', { fileId: activeFileId.value }).then(() => {
          getFiles(relative_id)
          activeFileId.value = 0
        }).catch(e => {
          showError.value = true
          errorTitle.value = "Error Deleting File"
          errorValue.value = `${e}`
        })

      }
    }).catch(e => {
      console.log(e)
    })
  }

  function pinFile(relative_id: number) {
    if (activeFile.value.pinned) {
      invoke('unpin_file', { fileId: activeFileId.value }).then(() => {
        getFiles(relative_id)
        activeFileId.value = 0
      }).catch(e => {
        showError.value = true
        errorTitle.value = "Error UnPining File"
        errorValue.value = `${e}`
      })
    } else {
      invoke('pin_file', { fileId: activeFileId.value }).then(() => {
        getFiles(relative_id)
        activeFileId.value = 0
      }).catch(e => {
        showError.value = true
        errorTitle.value = "Error Pining File"
        errorValue.value = `${e}`
      })
    }
  }

  return {
    files,
    activeFile,
    getFiles,
    createFile,
    activeFileId,
    deleteFile,
    pinFile
  }
})
