import { defineStore } from "pinia";
import { Image as Img } from "../utils/file";
import { type Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { errorValue, errorTitle, showError } from "../composables/error";
import { confirm } from "@tauri-apps/plugin-dialog";

export const useImagesStore = defineStore('images', () => {
  const images: Ref<Img[]> = ref([])
  const activeImageId = ref(0)
  function loadImages(relativeId: number) {
    invoke("images_by_relative_id", { relativeId: relativeId }).then(val => {
      images.value = val as Array<Img>
    }).catch(e => {
      showError.value = true
      errorTitle.value = "Error Loading Images"
      errorValue.value = `${e}`
    })
  }

  function createImage(imagePath: string, relativeId: number) {
    invoke('create_image', { params: { imagePath: imagePath, relativeId: relativeId } }).then(() => {
      loadImages(relativeId)
    }).catch(e => {
      showError.value = true
      errorTitle.value = "Error Saving Image"
      errorValue.value = `${e}`

    })
  }

  function pinImage(imageId: number) {
    invoke("pin_image", { imageId }).catch(e => {
      showError.value = true
      errorTitle.value = "Image Error"
      errorValue.value = `${e}`

    })
  }
  function deleteImage(imageId: number, relativeId: number) {
    confirm("This action is not reversible, are you sure?", { kind: 'warning' }).then(y => {
      if (y) {
        invoke("delete_image", { imageId }).then(() => {
          loadImages(relativeId)
          activeImageId.value = 0
        }).catch(e => {
          showError.value = true
          errorTitle.value = "Image Error"
          errorValue.value = `${e}`
        })
      }
    })
  }

  return {
    images,
    loadImages,
    createImage,
    pinImage,
    deleteImage,
    activeImageId
  }
})
