import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { ref, type Ref } from "vue";


type ParentRelative = {
  id: number
  firstName: string
  middleName: string
  lastName: string
}

/*
 
model: ["N/A", "Not Human", "Rejected by Them", "Rejected By Family", "Other"];
 */
export const useRelativesStore = defineStore('relatives', () => {

  const lostReasons = ref(["N/A", "Not Human", "Rejected by Them", "Rejected By Family", "Other"]);
  const femaleParents = ref([]) as Ref<Array<ParentRelative>>
  const maleParents = ref([]) as Ref<Array<ParentRelative>>
  function fetchMaleParents(relative_id: number) {
    invoke('male_parents', { relativeId: relative_id }).then(val => {
      maleParents.value = val as Array<ParentRelative>
    }).catch(e => {
      console.log(e)
    })
  }
  function fetchFemaleParants(relative_id: number) {
    invoke('female_parents', { relativeId: relative_id }).then(val => {
      femaleParents.value = val as Array<ParentRelative>
    }).catch(e => {
      console.log(e)
    })
  }

  return {
    femaleParents,
    lostReasons,
    maleParents,
    fetchMaleParents,
    fetchFemaleParants
  }
})
