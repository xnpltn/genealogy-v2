<script setup lang="ts">
import Aside from "./components/aside/index.vue"
import Females from "./components/tables/Females.vue";
import Relatives from "./components/tables/Relatives.vue";
import Employees from "./components/tables/Employees.vue";
import AddNewForm from "./components/AddNewForm.vue";
import NoteSection from "./components/NoteSection.vue";
import EditView from "./components/EditView.vue";
import ErrorModal from "./components/ErrorModal.vue";
import AboutModal from "./components/AboutModal.vue";
import { useStateStore } from "./store/state";
import { watch } from 'vue'

const stateStore = useStateStore()


watch(
  () => stateStore.darkTheme,
  (isDark) => {
    document.body.classList.toggle('dark-theme', isDark)
    if (isDark) {
      { document.body.style.backgroundColor = 'var(--clr-dark1)' }
    } else {
      { document.body.style.backgroundColor = 'var(--clr-light1)' }
    }
  },
  { immediate: true }
)

</script>

<template>
  <AboutModal />
  <ErrorModal />
  <main class="container">
    <Aside />
    <NoteSection v-if="stateStore.showNotes && stateStore.hasActiveRelative" />
    <div class="main">
      <div v-if="stateStore.activeTab == 0">
        <Relatives />
      </div>
      <div v-if="stateStore.activeTab == 1">
        <Females />
      </div>
      <div v-if="stateStore.activeTab == 2">
        <Employees />
      </div>
      <div v-if="stateStore.activeTab == 3">
        <AddNewForm />
      </div>
      <div v-if="stateStore.activeTab == 4">
        <EditView />
      </div>
    </div>
  </main>
</template>
