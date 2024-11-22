import { defineStore } from "pinia";
import { Image as Img } from "../utils/file";
import { type Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export const useImagesStore = defineStore('images', () => {
  const images: Ref<Img[]> = ref([])
  function loadImages(relativeId: number) {
    invoke("images_by_relative_id", { relativeId: relativeId }).then(val => {
      images.value = val as Array<Img>
      console.log(images.value)
    }).catch(e => {
      console.log(e)
    })
  }

  function createImage(imagePath: string, relativeId: number) {
    invoke('create_image', { params: { imagePath: imagePath, relativeId: relativeId } }).then(() => {
    }).catch(e => {
      console.log(e)
    })
  }

  return {
    images,
    loadImages,
    createImage
  }
})
