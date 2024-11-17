import { defineStore } from "pinia";
import { computed, ref } from "vue";

export const useStateStore = defineStore('relatives', () => {
  const showNotes = ref(false)
  const activeTab = ref(0)
  const activeRelativeId = ref(0)
  const hasActiveRelative = computed(() => activeRelativeId.value > 0)
  function setShowNotesToFalse() {
    showNotes.value = false
  }
  function setShowNotesToTrue() {
    showNotes.value = true
  }
  function changeActiveTab(id: number) {
    if (id != 4) {
      activeRelativeId.value = 0;
    }
    activeTab.value = id
  }
  function changeActiveRelativeId(id: number) {
    activeRelativeId.value = id
  }
  return {
    showNotes,
    activeTab,
    activeRelativeId,
    setShowNotesToFalse,
    setShowNotesToTrue,
    changeActiveTab,
    changeActiveRelativeId,
    hasActiveRelative,
  }
})
