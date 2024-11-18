import { defineStore } from "pinia";
import { type Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { type File } from "../utils/file";
import { open } from '@tauri-apps/plugin-dialog';
import { type CreateFileParams } from "../utils/file";



export const useFilesStore = defineStore('files', () => {
  const files: Ref<File[], File[]> = ref([])
  const activeFileId = ref(0)
  function getFiles(id: number) {
    invoke("files_by_relative_id", { activeRelativeId: id }).then((val) => {
      files.value = val as Array<File>
    }).catch(e => {
      if (e instanceof Error) {
        console.log(e.message, e.stack, e.name)
      } else {
        console.log(e)
      }
    })
  }

  function createFile(id: number) {
    open({ multiple: false, directory: false }).then((file) => {
      if (file) {
        let params: CreateFileParams = { filePath: file, relativeId: id }
        invoke("create_file", { params }).then(() => {
          getFiles(id)
        }).catch(e => {
          console.log(e)
        })
      }
      console.log(file);
    })
  }

  function deleteFile(relative_id: number) {
    console.log("del file ", activeFileId.value)
    invoke('delete_file', { fileId: activeFileId.value }).then(() => {
      getFiles(relative_id)
      activeFileId.value = 0
    }).catch(e => {
      console.log(e)
    })
  }

  return { files, getFiles, createFile, activeFileId, deleteFile }
})
