<script setup lang="ts">
import { useStateStore } from '../../store/state';
import { useNotesStore } from '../../store/notes';
import { useFilesStore } from '../../store/files';

const stateStore = useStateStore()
const notesStore = useNotesStore()
const filesStore = useFilesStore()
function resetState(tabIndex: number) {
  notesStore.activeNoteId = 0
  filesStore.activeFileId = 0
  stateStore.setShowNotesToFalse()
  stateStore.changeActiveTab(tabIndex)
}

function changeTheme() {
  stateStore.darkTheme = !stateStore.darkTheme
}
</script>

<template>
  <div class="header" :class="{ 'header-light': !stateStore.darkTheme, 'header-dark': stateStore.darkTheme }">
    <div class="header__tabs"
      :class="{ 'header__tabs-light': !stateStore.darkTheme, 'header__tabs-dark': stateStore.darkTheme }">
      <button :class="{ 'active': stateStore.activeTab == 0, }" class="btn" @click="; resetState(0);">Relatives</button>
      <button :class="{ 'active': stateStore.activeTab == 1 }" class="btn" @click="; resetState(1)">Females</button>
      <button :class="{ 'active': stateStore.activeTab == 2 }" class="btn" @click="; resetState(2)">Employees</button>
    </div>
    <div class="btn__containers"
      :class="{ 'btn__containers-light': !stateStore.darkTheme, 'btn__containers-dark': stateStore.darkTheme }">
      <button :class="{ 'header__btn-light': !stateStore.darkTheme, 'header__btn-dark': stateStore.darkTheme }"
        class="editButton" @click="; resetState(4)" v-if="stateStore.hasActiveRelative">Edit</button>
      <button :class="{ 'header__btn-light': !stateStore.darkTheme, 'header__btn-dark': stateStore.darkTheme }"
        class="editButton" @click=" resetState(3)">new
        relative</button>
      <button @click="changeTheme">{{ stateStore.darkTheme ? 'light' : 'dark' }}</button>
    </div>
  </div>
</template>


<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  border-bottom: 1px solid #dee2e6;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.header__tabs {
  display: flex;
  gap: 10px;
}

.btn {
  height: 40px;
  border: none;
  background-color: #495057;
  border-radius: 6px;
  color: white;
  font-size: 14px;
  cursor: pointer;
  padding: 0 15px;
  transition: all 0.3s ease;
  text-transform: capitalize;
}

.btn:hover {
  background-color: #6c757d;
}

.btn.active {
  background-color: #007bff;
  color: white;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.btn__containers {
  display: flex;
  align-items: center;
  gap: 10px;
}

.editButton {
  height: 40px;
  border: none;
  background-color: #28a745;
  border-radius: 6px;
  color: white;
  font-size: 14px;
  cursor: pointer;
  padding: 0 15px;
  transition: all 0.3s ease;
}

.editButton:hover {
  background-color: #218838;
}
</style>
