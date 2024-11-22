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
  <div class="navigation-header"
    :class="{ 'navigation-header--light': !stateStore.darkTheme, 'navigation-header--dark': stateStore.darkTheme }">
    <div class="navigation-tabs"
      :class="{ 'navigation-tabs--light': !stateStore.darkTheme, 'navigation-tabs--dark': stateStore.darkTheme }">
      <button :class="{
        'navigation-tab': true,
        'navigation-tab--light': !stateStore.darkTheme,
        'navigation-tab--dark': stateStore.darkTheme,
        'navigation-tab--active': stateStore.activeTab == 0
      }" @click="resetState(0)">
        Relatives
      </button>
      <button :class="{
        'navigation-tab': true,
        'navigation-tab--light': !stateStore.darkTheme,
        'navigation-tab--dark': stateStore.darkTheme,
        'navigation-tab--active': stateStore.activeTab == 1
      }" @click="resetState(1)">
        Females
      </button>
      <button :class="{
        'navigation-tab': true,
        'navigation-tab--light': !stateStore.darkTheme,
        'navigation-tab--dark': stateStore.darkTheme,
        'navigation-tab--active': stateStore.activeTab == 2
      }" @click="resetState(2)">
        Employees
      </button>
    </div>
    <div class="action-buttons"
      :class="{ 'action-buttons--light': !stateStore.darkTheme, 'action-buttons--dark': stateStore.darkTheme }">
      <button v-if="stateStore.hasActiveRelative" :class="{
        'action-button': true,
        'action-button--edit': true,
        'action-button--light': !stateStore.darkTheme,
        'action-button--dark': stateStore.darkTheme
      }" @click="resetState(4)">
        Edit
      </button>
      <button :class="{
        'action-button': true,
        'action-button--new': true,
        'action-button--light': !stateStore.darkTheme,
        'action-button--dark': stateStore.darkTheme
      }" @click="resetState(3)">
        New Relative
      </button>
      <button :class="{
        'theme-toggle': true,
        'theme-toggle--light': !stateStore.darkTheme,
        'theme-toggle--dark': stateStore.darkTheme
      }" @click="changeTheme">
        {{ stateStore.darkTheme ? 'Light' : 'Dark' }}
      </button>
    </div>
  </div>
</template>


<style scoped>
.navigation-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 15px;
  border-bottom: 1px solid var(--clr-frost3);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.navigation-header--light {
  background-color: var(--clr-light);
}

.navigation-header--dark {
  background-color: var(--clr-dark);
}

.navigation-tabs {
  display: flex;
  gap: 10px;
}

.navigation-tabs--light {
  background-color: var(--clr-light);
}

.navigation-tabs--dark {
  background-color: var(--clr-dark);
}

.navigation-tab {
  height: 40px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  padding: 0 15px;
  transition: all 0.3s ease;
  text-transform: capitalize;
}

.navigation-tab--light {
  background-color: var(--clr-light2);
  color: var(--clr-dark);
}

.navigation-tab--dark {
  background-color: var(--clr-dark2);
  color: var(--clr-light);
}

.navigation-tab:hover {
  background-color: var(--clr-frost2);
  color: var(--clr-light);
}

.navigation-tab--active {
  background-color: var(--clr-frost);
  color: var(--clr-light);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.action-buttons {
  display: flex;
  align-items: center;
  gap: 10px;
}

.action-buttons--light {
  background-color: var(--clr-light);
}

.action-buttons--dark {
  background-color: var(--clr-dark);
}

.action-button {
  height: 40px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  padding: 0 15px;
  transition: all 0.3s ease;
}

.action-button--light {
  background-color: var(--clr-frost3);
  color: var(--clr-dark);
}

.action-button--dark {
  background-color: var(--clr-frost);
  color: var(--clr-light);
}

.action-button:hover {
  background-color: var(--clr-frost2);
  color: var(--clr-light);
}

.theme-toggle {
  height: 40px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  padding: 0 15px;
  transition: all 0.3s ease;
}

.theme-toggle--light {
  background-color: var(--clr-dark2);
  color: var(--clr-light);
}

.theme-toggle--dark {
  background-color: var(--clr-light2);
  color: var(--clr-dark);
}

.theme-toggle:hover {
  background-color: var(--clr-frost2);
  color: var(--clr-light);
}
</style>
